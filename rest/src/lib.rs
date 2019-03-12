use futures::future::Future;
use reqwest::header::HeaderMap;
use reqwest::r#async::{Client as ReqwestClient, ClientBuilder};

use spectacles_model::message::{Message, MessageBuilder};

pub use crate::errors::{Error, Result};

mod errors;
mod views;
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
    pub fn new(token: String, use_ratelimit_bucket: bool) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", token.parse().unwrap());
        let client = ClientBuilder::new().default_headers(headers).build()?;

       Ok(RestClient {
            token,
            base_url: constants::BASE_URL.to_string(),
            using_ratelimiter: use_ratelimit_bucket,
            http: client
        })
    }

    pub fn set_base(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    pub fn create_message(&self, channel_id: String, payload: MessageBuilder) -> impl Future<Item = Message, Error = Error> {
        let message = self.http.post(
            format!("{}/channels/{}/messages", constants::BASE_URL, channel_id).as_str()
        ).json(&payload).send();
        message.and_then(|mut resp| resp.json::<Message>()).map_err(Error::from)
    }
}