use futures::future::Future;
use reqwest::r#async::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::constants::BASE_URL;
use crate::errors::Error;

pub use self::channel::*;
pub use self::emoji::*;
pub use self::guild::*;
pub use self::invite::*;
pub use self::user::*;
pub use self::webhook::*;

mod channel;
mod emoji;
mod guild;
mod invite;
mod user;
mod webhook;

/// Handles the routing of requests to the Discord API.
#[derive(Clone, Debug)]
pub struct RouteManager {
    pub http: ReqwestClient,
}

impl RouteManager {
    fn get<T>(&self, route: String) -> impl Future<Item=T, Error=Error>
        where T: DeserializeOwned + Send + 'static
    {
        let url = format!("{}{}", BASE_URL, route);
        self.http.get(url.as_str()).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    fn post<S, T>(&self, route: String, body: S) -> impl Future<Item=T, Error=Error>
        where S: Serialize + Send,
        T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.http.post(url.as_str()).json(&body).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    fn patch<S, T>(&self, route: String, body: S) -> impl Future<Item=T, Error=Error>
        where S: Serialize + Send,
        T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.http.patch(url.as_str()).json(&body).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    fn put<S, T>(&self, route: String, body: S) -> impl Future<Item=T, Error=Error>
        where S: Serialize + Send,
        T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.http.put(url.as_str()).json(&body).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    fn delete<T>(&self, route: String) -> impl Future<Item=T, Error=Error>
        where T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.http.delete(url.as_str()).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }
}