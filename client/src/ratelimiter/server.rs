use std::collections::HashMap;
use std::ops::Sub;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use chrono::offset::TimeZone;
use futures::future::{Future, Loop};
use futures::stream::Stream;
use hyper::{Body, HeaderMap, Method, Request, Response, Server, Uri, Version};
use hyper::header::HeaderValue;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use parking_lot::{Mutex, RwLock};
use tokio::timer::Delay;

use crate::errors::Error;
use crate::ratelimiter::bucket::Bucket;

use super::RatelimitOptions;

type BucketMap = RwLock<HashMap<String, Arc<Mutex<Bucket>>>>;
type GlobalMutex = Mutex<Option<DateTime<Utc>>>;

pub struct RatelimitServer {
    config: Arc<Mutex<RatelimitOptions>>,
    state: Arc<RatelimitState>,
}

pub struct RatelimitState {
    buckets: Arc<BucketMap>,
    global: Arc<GlobalMutex>,
}

struct RequestState {
    ratelimiter: Arc<RatelimitState>,
    proxy_url: String,
    method: Method,
    uri: Uri,
    version: Version,
    headers: HeaderMap<HeaderValue>,
}

#[derive(Deserialize, Debug, Clone)]
struct RatelimitResponse {
    message: String,
    retry_after: u64,
    global: bool,
}

pub enum ResponseStatus {
    Success(Response<Body>),
    Ratelimited,
    ServerError,
}

impl RatelimitServer {
    pub fn new(opts: RatelimitOptions) -> Self {
        Self {
            config: Arc::new(Mutex::new(opts)),
            state: Arc::new(RatelimitState {
                buckets: Arc::new(BucketMap::new(HashMap::new())),
                global: Arc::new(GlobalMutex::new(None)),
            }),
        }
    }

    pub fn start(&self) {
        let address = self.config.lock().address;
        let parent_url = self.config.lock().url.clone();
        let parent_state = Arc::clone(&self.state);

        let make_svc = make_service_fn(move |socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            let loop_state = Arc::clone(&parent_state);
            let loop_url = parent_url.clone();

            service_fn(move |req: Request<Body>| {
                let (parts, body) = req.into_parts();
                let orig_state = Arc::new(RequestState {
                    ratelimiter: Arc::clone(&loop_state),
                    method: parts.method,
                    headers: parts.headers,
                    version: parts.version,
                    uri: parts.uri,
                    proxy_url: loop_url.clone(),
                });

                body.concat2().from_err().map(|chunk| chunk.to_vec()).and_then(move |bytes| {
                    futures::future::loop_fn(Arc::clone(&orig_state), move |state| {
                        let current_state = Arc::clone(&state);
                        let resp_state = Arc::clone(&state.ratelimiter);
                        let continue_state = Arc::clone(&current_state);
                        let path = current_state.uri.path().to_owned();
                        let route = Bucket::make_route(&current_state.method, path.clone());
                        let mut new_req = Request::builder()
                            .method(&state.method)
                            .uri(&state.uri)
                            .version(state.version)
                            .body({
                                match state.method {
                                    Method::GET => Body::default(),
                                    _ => Body::from(bytes.clone())
                                }
                            })
                            .expect("Failed to construct proxy request");

                        let old_headers = state.headers.clone();
                        let mut new_headers = HeaderMap::new();
                        if old_headers.contains_key("Authorization") {
                            new_headers.insert("Authorization", old_headers["Authorization"].clone());
                        };
                        if old_headers.contains_key("User-Agent") {
                            new_headers.insert("User-Agent", old_headers["User-Agent"].clone());
                        };
                        new_headers.insert("Content-Type", old_headers["Content-Type"].clone());
                        *new_req.headers_mut() = new_headers;

                        enqueue(route.clone(), Arc::clone(&current_state.ratelimiter))
                            .and_then(move |_| hyper_reverse_proxy::call(remote_addr.ip(), &current_state.proxy_url, new_req).from_err())
                            .and_then(|resp| process_response(route, resp, resp_state))
                            .map(|status| match status {
                                ResponseStatus::Success(resp) => Loop::Break(resp),
                                ResponseStatus::Ratelimited | ResponseStatus::ServerError => Loop::Continue(continue_state)
                            })
                    })
                })
            })
        });


        info!("HTTP ratelimiter proxy starting at address {:?}", address);

        hyper::rt::run(Server::bind(&address)
            .serve(make_svc)
            .map_err(|e| error!("Failed to start server: {:?}", e))
        );

    }
}

pub fn enqueue(path: String, state: Arc<RatelimitState>) -> Box<Future<Item=(), Error=Error> + Send> {
    let buckets = Arc::clone(&state.buckets);
    let global = Arc::clone(&state.global);
    let mut routes = buckets.write();
    let bucket = routes.entry(path.clone())
        .or_insert(Arc::new(Mutex::new(Bucket::new(path))));

    if global.lock().is_some() {
        let global = global.lock().take().unwrap();
        let duration = global.sub(Utc::now()).to_std().unwrap();
        warn!("Reached global ratelimit, slowing down request.");

        Box::new(Delay::new(Instant::now() + duration).map_err(Error::from))
    } else if bucket.lock().remaining <= 0 {
        let ready = bucket.lock().take();
        match ready {
            Some(_) => {
                warn!("Reached route-level ratelimit, slowing down request.");
                let bkt = Arc::clone(&bucket);
                let reset = bkt.lock().reset.unwrap();
                let duration = reset.sub(Utc::now()).to_std().unwrap();
                Box::new(Delay::new(Instant::now() + duration)
                    .map_err(Error::from)
                    .map(move |_| {
                        let mut curr = bkt.lock();
                        curr.remaining = curr.limit;
                    })
                )
            },
            None => Box::new(futures::future::ok(()))
        }
    } else {
        Box::new(futures::future::ok(()))
    }
}

pub fn process_response(path: String, resp: Response<Body>, state: Arc<RatelimitState>) -> Box<Future<Item=ResponseStatus, Error=Error> + Send> {
    let buckets = Arc::clone(&state.buckets);
    let mut routes = buckets.write();
    let bucket = routes.entry(path.clone())
        .or_insert(Arc::new(Mutex::new(Bucket::new(path))));

    let headers = resp.headers();
    if headers.contains_key("x-ratelimit-limit") {
        (*bucket.lock()).limit = headers["x-ratelimit-limit"].to_str()
            .unwrap()
            .parse::<i64>()
            .expect("Failed to parse ratelimit limit header")
    };
    if headers.contains_key("x-ratelimit-remaining") {
        (*bucket.lock()).remaining = headers["x-ratelimit-remaining"].to_str()
            .unwrap()
            .parse::<i64>()
            .expect("Failed to parse ratelimit remaining header")
    };

    let status = resp.status();
    if status.is_server_error() {
        Box::new(Delay::new(Instant::now() + Duration::from_secs(5))
            .map_err(Error::from)
            .map(|_| ResponseStatus::ServerError)
        )
    } else if status.as_u16() == 429 {
        let global = Arc::clone(&state.global);
        let bkt = Arc::clone(&bucket);
        Box::new(resp.into_body().concat2().map_err(Error::from)
            .and_then(|body| serde_json::from_slice::<RatelimitResponse>(&body).map_err(Error::from))
            .map(move |body| {
                let duration = chrono::Duration::from_std(Duration::from_millis(body.retry_after)).unwrap();
                let reset = Utc::now()
                    .checked_add_signed(duration);
                if body.global {
                    *global.lock() = reset;
                } else {
                    (*bkt.lock()).reset = reset
                };

                ResponseStatus::Ratelimited
            })
        )
    } else {
        let bucket = Arc::clone(&bucket);
        if headers.contains_key("x-ratelimit-reset") {
            let reset_time = headers["x-ratelimit-reset"]
                .to_str()
                .unwrap()
                .parse::<i64>()
                .expect("Failed to parse ratelimit reset header");
            let date = headers["date"].to_str()
                .unwrap()
                .replace("GMT", "+0000");
            let parsed = DateTime::parse_from_str(&date, "%a, %d %b %Y %T %z")
                .unwrap()
                .timestamp();
            let current = Utc::now().timestamp();
            let diff = current - parsed;
            (*bucket.lock()).reset = Some(Utc.timestamp(reset_time + diff, 0));

            Box::new(futures::future::ok(ResponseStatus::Success(resp)))
        } else {
            Box::new(futures::future::ok(ResponseStatus::Success(resp)))
        }
    }
}
