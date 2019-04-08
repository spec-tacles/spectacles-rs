use std::borrow::Cow;
use std::collections::vec_deque::VecDeque;

use chrono::{DateTime, Utc};
use futures::sync::oneshot::{self, Receiver, Sender};
use hyper::Method;
use regex::{Captures, Regex};

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
    pub fn make_route(method: &Method, path: String) -> String {
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

    pub(crate) fn take(&mut self) -> Option<Receiver<()>> {
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