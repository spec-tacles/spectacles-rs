use futures::future::Future;
use reqwest::header::HeaderMap;
use reqwest::r#async::Client as ReqwestClient;

use spectacles_model::message::{Message, MessageBuilder};

pub use crate::errors::{Error, Result};

mod errors;
mod routes;
mod constants;

/// The Main client which is used to interface with the various components of the Discord API.
pub struct RestClient {
    /// The bot token for this user.
    pub token: String,
    /// The base URL of the client. This may be changed to accomodate an external proxy system.
    pub base_url: String,
    /// Whether or not the default ratelimit bucket will be used for all requests.
    /// If this is set to false, it is the user's responsibility to ensure that all requests to the Discord API are properly ratelimited.
    pub using_ratelimiter: bool,
    http: ReqwestClient,
}

impl RestClient {
    /// Creates a new REST client with the
    pub fn new(token: String, use_ratelimit_bucket: bool) -> Self {
        RestClient {
            token,
            base_url: constants::BASE_URL.to_string(),
            using_ratelimiter: use_ratelimit_bucket,
            http: ReqwestClient::new()
        }
    }

    pub fn set_base(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }
}