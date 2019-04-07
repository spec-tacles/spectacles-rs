#![feature(futures_api, async_await, await_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[warn(rust_2018_idioms)]
#[macro_use]
extern crate serde_json;

use std::sync::Arc;

use futures::future::{Future, Loop};
use http::header::HeaderValue;
use parking_lot::Mutex;
use reqwest::header::HeaderMap;
use reqwest::Method;
use reqwest::r#async::{
    Client as ReqwestClient,
    ClientBuilder,
    multipart::Form,
};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;

pub(crate) use ratelimit::*;
use spectacles_model::snowflake::Snowflake;
/// A collection of interfaces for endpoint-specific Discord objects.
pub use views::*;

use crate::constants::BASE_URL;
pub use crate::errors::{Error, Result};

mod errors;
mod ratelimit;
mod views;
mod constants;

/// The Main client which is used to interface with the various components of the Discord API.
#[derive(Clone, Debug)]
pub struct RestClient {
    /// The bot token for this user.
    pub token: String,
    /// The base URL of the client. This may be changed to accomodate an external proxy system.
    pub base_url: String,
    pub http: ReqwestClient,
    ratelimiter: Option<Arc<Mutex<Ratelimter>>>,
}

impl RestClient {
    /// Creates a new REST client with the provided configuration.
    /// The second argument denotes whether or not to use the built-in rate limiter to rate limit requests to the Discord API.
    /// If this is set to false, it is the USER's responsibility to ensure that all requests to the Discord API are properly rate limited.
    pub fn new(token: String, using_ratelimiter: bool) -> Self {
        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };
        let mut headers = HeaderMap::new();
        let value = HeaderValue::from_str(&token).unwrap();
        let agent = HeaderValue::from_str(
            "DiscordBot (https://github.com/spec-tacles/spectacles-rs, v1.0.0)"
        ).unwrap();
        headers.insert("Authorization", value);
        headers.insert("User-Agent", agent);

        let client = ClientBuilder::new().default_headers(headers).build()
            .expect("Failed to build HTTP client");

        let mut rest = RestClient {
            token,
            http: client.clone(),
            base_url: constants::BASE_URL.to_string(),
            ratelimiter: None,
        };

        if using_ratelimiter {
            rest.ratelimiter = Some(Arc::new(Mutex::new(Ratelimter::new(client))));
        };

        rest
    }

    /// Changes the base URL for all requests that are made to the Discord API.
    /// This method is most commonly used to configure an external ratelimiter service.
    pub fn set_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// Opens a ChannelView for the provided Channel snowflake.
    pub fn channel(&self, id: &Snowflake) -> ChannelView {
        ChannelView::new(id.0, self.clone())
    }
    /// Opens a GuildView for the provided Guild snowflake.
    pub fn guild(&self, id: &Snowflake) -> GuildView {
        GuildView::new(id.0, self.clone())
    }

    /// Makes an HTTP request to the provided Discord API endpoint.
    pub fn request<T>(&self, endpt: Endpoint) -> Box<Future<Item=T, Error=Error> + Send>
        where T: DeserializeOwned + Send + 'static
    {
        if let Some(ref rl) = self.ratelimiter {
            let http = self.http.clone();
            Box::new(futures::future::loop_fn(Arc::clone(rl), move |ratelimit| {
                let route = Bucket::make_route(endpt.method.clone(), endpt.url.clone());
                let req = http.request(endpt.method.clone(), &endpt.url)
                    .query(&endpt.query)
                    .json(&endpt.json);
                let limiter = Arc::clone(&ratelimit);
                let limiter_2 = Arc::clone(&limiter);
                ratelimit.lock().enqueue(route.clone())
                    .and_then(|_| req.send().from_err())
                    .and_then(move |resp| limiter.lock().handle_resp(route, resp))
                    .map(move |status| match status {
                        ResponseStatus::Success(resp) => Loop::Break(resp),
                        ResponseStatus::Ratelimited | ResponseStatus::ServerError => Loop::Continue(limiter_2)
                    })
            }).and_then(|mut resp| resp.json().from_err()))
        } else {
            let req = self.http.request(endpt.method.clone(), &endpt.url)
                .query(&endpt.query)
                .json(&endpt.json);
            Box::new(req.send().map_err(Error::from)
                .and_then(|mut resp| resp.json().from_err())
            )
        }
    }
}

/// A structure representing a Discord API endpoint, in the context of an HTTP request.
#[derive(Debug)]
pub struct Endpoint {
    url: String,
    method: Method,
    json: Option<Value>,
    query: Option<Value>,
    multipart: Option<Form>,
}

impl Endpoint {
    /// Creates a new endpoint from the following HTTP method and URL string.
    pub fn new(method: Method, url: String) -> Self {
        Self {
            method,
            url: format!("{}{}", BASE_URL, url),
            json: None,
            query: None,
            multipart: None,
        }
    }

    /// Adds a json body to the request.
    pub fn json<T: Serialize>(mut self, payload: T) -> Endpoint {
        match serde_json::to_value(payload) {
            Ok(val) => self.json = Some(val),
            Err(_) => self.json = None
        };

        self
    }

    /// Adds a query parameter to the endpoint.
    pub fn query<T: Serialize>(mut self, payload: T) -> Endpoint {
        match serde_json::to_value(payload) {
            Ok(val) => self.query = Some(val),
            Err(_) => self.query = None
        };

        self
    }

    /// Adds a multipart form to the endpoint.
    pub fn multipart(mut self, payload: Form) -> Endpoint {
        self.multipart = Some(payload);
        self
    }
}
