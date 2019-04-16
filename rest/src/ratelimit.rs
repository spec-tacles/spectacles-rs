use std::borrow::Cow;
use std::collections::vec_deque::VecDeque;
use std::ops::Sub;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use chrono::offset::TimeZone;
use futures::Future;
use futures::sync::oneshot::{self, Receiver, Sender};
use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock};
use regex::{Captures, Regex};
use reqwest::Method;
use reqwest::r#async::{Client as ReqwestClient, Response};
use tokio::timer::Delay;

use crate::Error;
use crate::errors::APIError;

#[derive(Deserialize, Debug, Clone)]
struct RatelimitResponse {
    message: String,
    retry_after: u64,
    global: bool,
}

/// A utility for rate limiting requests made to the Discord API.
#[derive(Debug)]
pub struct Ratelimter {
    /// The underlying client instance.
    pub http: ReqwestClient,
    /// A collection of rate limit buckets, mapped by route.
    pub buckets: Arc<RwLock<HashMap<String, Arc<Mutex<Bucket>>>>>,
    /// The reset time for the global Discord rate limit.
    global: Arc<Mutex<Option<DateTime<Utc>>>>,
}

pub enum ResponseStatus {
    Success(Response),
    Ratelimited,
    ServerError,
}

#[derive(Deserialize)]
struct ErrorResponse {
    code: i32,
    message: String,
}


impl Ratelimter {
    pub fn new(http: ReqwestClient) -> Self {
        Self {
            http,
            buckets: Arc::new(RwLock::new(HashMap::new())),
            global: Arc::new(Mutex::new(None)),
        }
    }

    pub fn enqueue(&mut self, path: String) -> Box<Future<Item=(), Error=Error> + Send> {
        let buckets = Arc::clone(&self.buckets);
        let mut routes = buckets.write();
        let bucket = routes.entry(path.clone())
            .or_insert(Arc::new(Mutex::new(Bucket::new(path))));

        if self.global.lock().is_some() {
            let global = self.global.lock().take().unwrap();
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
                    let duration = reset.sub(Utc::now()).to_std().unwrap_or(Duration::from_secs(0));
                    Box::new(Delay::new(Instant::now() + duration)
                        .map_err(Error::from)
                        .map(move |_| {
                            let mut curr = bkt.lock();
                            curr.remaining = curr.limit;
                        })
                    )
                }

                None => Box::new(futures::future::ok(()))
            }
        } else {
            Box::new(futures::future::ok(()))
        }
    }

    pub(crate) fn handle_resp(&mut self, path: String, mut resp: Response) -> Box<Future<Item=ResponseStatus, Error=Error> + Send> {
        let buckets = Arc::clone(&self.buckets);
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
            let global = Arc::clone(&self.global);
            let bkt = Arc::clone(&bucket);
            Box::new(resp.json::<RatelimitResponse>().from_err().map(move |body| {
                let duration = chrono::Duration::from_std(Duration::from_millis(body.retry_after)).unwrap();
                let reset = Utc::now()
                    .checked_add_signed(duration);
                if body.global {
                    *global.lock() = reset;
                } else {
                    (*bkt.lock()).reset = reset
                };

                ResponseStatus::Ratelimited
            }))
        } else if status.is_client_error() {
            Box::new(resp.json::<ErrorResponse>().from_err()
                .and_then(move |body| {
                    futures::future::err(Error::Discord(APIError {
                        code: body.code,
                        message: body.message,
                        http_status: resp.status(),
                    }))
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
}

/// A rate limiter bucket used for maintaining Discord rate limits.
#[derive(Debug)]
pub struct Bucket {
    /// A queue of requests that are being ratelimited.
    pub queue: VecDeque<Sender<()>>,
    /// The remaining time left for this request.
    pub remaining: i64,
    /// The route that this bucket is for.
    pub route: String,
    /// The request limit.
    pub limit: i64,
    /// The time in which the ratelimit resets.
    pub reset: Option<DateTime<Utc>>,
}

impl Bucket {
    /// Creates a bucket route to be used as a storage key.
    #[allow(unused_assignments)]
    pub fn make_route(method: Method, path: String) -> String {
        let default_regex = Regex::new(r"/([a-z-]+)/(?:[0-9]{17,19})/g").unwrap();
        let reaction_regex = Regex::new(r"/reactions/[^/]+/g").unwrap();
        let webhook_regex = Regex::new(r"^/webhooks/(\d+)/[A-Za-z0-9-_]{64,}/").unwrap();

        let mut route = default_regex.replace(&path, |matches: &Captures| {
            let mat = matches.get(1).unwrap();
            match mat.as_str() {
                "channels" | "guilds" | "webhooks" => mat.as_str().to_owned(),
                _ => format!("/{}/:id", mat.as_str())
            }
        });

        let owned = route.into_owned();
        route = reaction_regex.replace(owned.as_str(), "/reactions/:id");
        route = webhook_regex.replace(owned.as_str(), "/webhooks/$1/:token");

        if method == Method::DELETE && route.ends_with("/messages/:id") {
            let formatted = format!("{}{}", method.as_str(), route);
            route = Cow::from(formatted);
        }

        route.into_owned()
    }

    fn take(&mut self) -> Option<Receiver<()>> {
        if self.reset.is_some() {
            let (tx, rx) = oneshot::channel();

            self.queue.push_back(tx);

            Some(rx)
        } else {
            None
        }
    }

    pub(crate) fn new(route: String) -> Self {
        Self {
            queue: VecDeque::new(),
            route,
            limit: 1,
            remaining: 1,
            reset: None,
        }
    }
}