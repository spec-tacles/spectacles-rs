use http::header::HeaderValue;
use reqwest::header::HeaderMap;
use reqwest::r#async::ClientBuilder;

pub use bucket::*;
use routes::*;

pub use crate::errors::{Error, Result};

mod errors;
mod bucket;
mod routes;
mod constants;

/// The Main client which is used to interface with the various components of the Discord API.
#[derive(Clone, Debug)]
pub struct RestClient {
    /// The bot token for this user.
    pub token: String,
    /// The base URL of the client. This may be changed to accomodate an external proxy system.
    pub base_url: String,
    /// Whether or not the default ratelimit bucket will be used for all requests.
    /// If this is set to false, it is the user's responsibility to ensure that all requests to the Discord API are properly ratelimited.
    pub using_ratelimiter: bool,
    /// The route manager for the client.
    pub router: RouteManager
}

impl RestClient {
    /// Creates a new REST client with the provided options.
    pub fn new(token: String, using_ratelimiter: bool) -> Self {
        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };
        let mut headers = HeaderMap::new();
        let value = HeaderValue::from_str(token.as_str()).unwrap();
        headers.insert("Authorization", value);
        let client = ClientBuilder::new().default_headers(headers).build()
            .expect("Failed to build HTTP client");

        RestClient {
            token,
            base_url: constants::BASE_URL.to_string(),
            using_ratelimiter,
            router: RouteManager { http: client }
        }
    }

    /// Changes the base URL for all requests that are made to the Discord API.
    /// This method is most commonly used to configure an external ratelimiter service.
    pub fn set_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    pub fn channel(&self, id: impl Into<u64>) -> ChannelsView {
        ChannelsView { id: id.into(), router: self.router.clone() }
    }
}