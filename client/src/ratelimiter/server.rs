use std::collections::HashMap;
use std::ops::Sub;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use chrono::offset::TimeZone;
use futures::future::{Future, Loop};
use futures::stream::Stream;
use hyper::{Body, Request, Response, Server};
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

struct RatelimitState {
    buckets: Arc<BucketMap>,
    global: Arc<GlobalMutex>,
}

#[derive(Deserialize, Debug, Clone)]
struct RatelimitResponse {
    message: String,
    retry_after: u64,
    global: bool,
}

enum ResponseStatus {
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
                let method = req.method();
                let uri = req.uri();
                let path = uri.path().to_owned();
                let route = Bucket::make_route(method, path.clone());
                let parent_req = hyper_reverse_proxy::call(
                    remote_addr.ip(),
                    &loop_url,
                    req,
                ).from_err().shared();
                futures::future::loop_fn(Arc::clone(&loop_state), move |fresh_state| {
                    let path = path.clone();
                    let proxy = parent_req.clone();
                    let req_state = Arc::clone(&fresh_state);
                    let resp_state = Arc::clone(&req_state);
                    let continue_state = Arc::clone(&resp_state);
                    enqueue(path.clone(), req_state)
                        .and_then(move |_| proxy.map_err(|err| Error::from(err.deref())))
                        .and_then(|resp| process_response(path, resp, resp_state))
                        .map(|status| match status {
                            ResponseStatus::Success(resp) => Loop::Break(resp),
                            ResponseStatus::Ratelimited | ResponseStatus::ServerError => Loop::Continue(continue_state)
                        })
                })
            })
        });

        let server = Server::bind(&address)
            .serve(make_svc)
            .map_err(|e| error!("Failed to start server: {:?}", e));

        info!("HTTP ratelimit has started proxy on {:?}", address);

        hyper::rt::run(server);
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
