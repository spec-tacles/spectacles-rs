//! # Spectacles REST
//! Spectacles REST offers a simple, easy-to-use client for making HTTP requests to the Discord API.
//! All HTTP requests are made asynchronously and never block the thread, with the help of the Tokio runtime.
//!
//! ## Creating a Client
//! A [`RestClient`], resoponsible for performing all requests, can be easily constructed with the `new` function.
//! ```rust
//! use spectacles_rest::RestClient;
//! let token = std::env::var("DISCORD_TOKEN").expect("Failed to parse token");
//! let rest = RestClient::new(token, true);
//! ```
//! The client accepts a boolean as a second parameter, which determines whether or not the internal rate limiter will be used on each request.
//!
//! ## Views
//! The Client ships with three views for specific endpoints of the Discord API.
//! The [`ChannelView`] provides a set of methods for interacting with a Discord channel.
//! The [`GuildlView`] provides a set of methods for interacting with a Discord guild, or "server".
//! The [`WebhookView`] provides a set of methods for interacting with Discord webhooks.
//!
//! [`ChannelView`]: struct.ChannelView.html
//! [`GuildView`]: struct.GuildView.html
//! [`WebhookView`]: struct.WebhookView.html
//! [`RestClient`]: struct.RestClient.html
//!
//! Here is a brief example of sending a message to a Discord channel using the ChannelView.
//! ```rust
//! #![feature(futures_api, async_await, await_macro)]
//! #[macro_use] extern crate tokio;
//! use spectacles_rest::RestClient;
//! use spectacles_model::Snowflake;
//!
//! fn main() {
//!     // ~snip~
//!     // Initialize the Rest Client, with a token.
//!     let rest = RestClient::new(token, true);
//!     tokio::run_async(async {
//!         // We create a Snowflake object from an ID that we provide, passing it to the ChannelView.
//!         await!(rest.channel(&Snowflake(CHANNEL_ID_HERE)).create_message("Hello World."))
//!             .expect("Failed to send message to Discord");
//!     });
//! }
//! ```
//! ## Rate Limiting
//! As mentioned earlier, the library includes an in-memory rate limiter bucket system for preemptively managing Discord Ratelimits.
//! This is sufficient if you do not plan on accessing the Discord API from a single server.
//! If you plan to make requests in a distributed fashion, you will need to make use of an external state for keeping track of rate limits.
//! The library currently supports using a custom Discord proxy, featured in the Spectacles client, to be used as a central hub for handling requests.
//! ```rust
//! use spectacles_rest::RestClient;
//! let proxy = std::env::var("PROXY").expect("Failed to parse proxy URL.");
//! let rest = RestClient::new(token, false) // false tells the client to skip the default in memory rate limiter.
//!     .set_proxy(proxy);
//!
//! ```
//! More rate limiting strategies will be considered in future.
//!
//!
//! ## Installation
//! Simply add the package to your Cargo.toml file.
//! ```toml
//! [dependencies]
//! spectacles-rest = "0.1.0"
//! ```
//!
//! ```

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
use spectacles_model::channel::Channel;
use spectacles_model::guild::{CreateGuildOptions, Guild};
use spectacles_model::invite::Invite;
use spectacles_model::snowflake::Snowflake;
use spectacles_model::User;
use spectacles_model::voice::VoiceRegion;
/// A collection of interfaces for endpoint-specific Discord objects.
pub use views::*;

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
    /// If you plan to use a distributed architecture, you will need an external ratelimiter to ensure ratelimis are kept across servers.
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

    /// Enables support for routing all requests though an HTTP rate limiting proxy.
    /// If you plan on making distributed REST requests, an HTTP proxy is recommended for handling rate limits in a distributed manner.
    pub fn set_proxy(mut self, url: String) -> Self {
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

    /// Opens a WebhookView for the provided Webhook snowflake.
    pub fn webhook(&self, id: &Snowflake) -> WebhookView {
        WebhookView::new(id.0, self.clone())
    }

    /// Gets a User object for the provided snowflake.
    pub fn get_user(&self, id: &Snowflake) -> impl Future<Item=User, Error=Error> {
        self.request(Endpoint::new(
            Method::GET,
            format!("/users/{}", id.0),
        ))
    }

    /// Opens a new DM channel with the user at the provided user ID.
    pub fn create_dm(&self, user: &Snowflake) -> impl Future<Item=Channel, Error=Error> {
        let json = json!({
            "recipient_id": user.0
        });

        self.request(Endpoint::new(
            Method::POST,
            String::from("/users/@me/channels"),
        ).json(json))
    }

    /// Creates a new guild, setting the current client user as owner.
    /// This endpoint may only be used for bots who are in less than 10 guilds.
    pub fn create_guild(&self, opts: CreateGuildOptions) -> impl Future<Item=Guild, Error=Error> {
        self.request(Endpoint::new(
            Method::POST,
            String::from("/guilds"),
        ).json(opts))
    }

    /// Leaves the guild using the provided guild ID.
    pub fn leave_guild(&self, id: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.request(Endpoint::new(
            Method::DELETE,
            format!("/users/@me/guilds/{}", id.0),
        ))
    }


    /// Modifies properties for the current user.
    /*pub fn modify_current_user(&self) -> impl Future<Item = User, Error = Error> {
        self.client.request(Endpoint::new(
            Method::PATCH,
            String::from("/users/@me)",
        ))
    }*/

    /// Obtains a list of Discord voice regions.
    pub fn get_voice_regions(&self) -> impl Future<Item=Vec<VoiceRegion>, Error=Error> {
        self.request(Endpoint::new(
            Method::GET,
            String::from("/voice/regions"),
        ))
    }

    /// Obtains an invite object from Discord using the given code.
    /// The second argument denotes whether the invite should contain approximate member counts
    pub fn get_invite(&self, code: &str, member_counts: bool) -> impl Future<Item=Invite, Error=Error> {
        self.request(Endpoint::new(
            Method::GET,
            format!("/invites/{}?with_counts={}", code, member_counts),
        ))
    }

    /// Deletes this invite from the its parent channel.
    /// This requires that the client have the `MANAGE_CHANNELS` permission.
    pub fn delete_invite(&self, code: &str) -> impl Future<Item=Invite, Error=Error> {
        self.request(Endpoint::new(
            Method::GET,
            format!("/invites/{}", code),
        ))
    }

    /// Makes an HTTP request to the provided Discord API endpoint.
    /// Depending on the ratelimiter status, the request may or may not be rate limited.
    pub fn request<T>(&self, mut endpt: Endpoint) -> Box<Future<Item=T, Error=Error> + Send>
        where T: DeserializeOwned + Send + 'static
    {
        if let Some(ref rl) = self.ratelimiter {
            let http = self.http.clone();
            let base = self.base_url.clone();
            Box::new(futures::future::loop_fn(Arc::clone(rl), move |ratelimit| {
                let req_url = format!("{}{}", base, &endpt.url);
                let route = Bucket::make_route(endpt.method.clone(), req_url.clone());
                let mut req = http.request(endpt.method.clone(), &req_url)
                    .query(&endpt.query)
                    .json(&endpt.json);
                if endpt.multipart.is_some() {
                    let form = endpt.multipart.take().unwrap();
                    req = req.multipart(form);
                };
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
            let req_url = format!("{}{}", self.base_url, &endpt.url);
            let req = self.http.request(endpt.method.clone(), &req_url)
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
            url,
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

    /// Adds a multipart form to the endpoint, which is useful for sending files to the Discord API.
    pub fn multipart(mut self, payload: Form) -> Endpoint {
        self.multipart = Some(payload);
        self
    }
}
