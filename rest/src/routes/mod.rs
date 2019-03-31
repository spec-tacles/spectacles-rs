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

pub trait View {
    fn route_str(&self) -> String;
}

/// Handles the routing of requests to the Discord API.
#[derive(Clone, Debug)]
pub struct RouteManager(pub ReqwestClient);

impl RouteManager {
    pub fn get<T>(&self, route: String) -> impl Future<Item=T, Error=Error>
        where T: DeserializeOwned + Send + 'static
    {
        let url = format!("{}{}", BASE_URL, route);
        self.0.get(url.as_str()).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    pub fn post<S, T>(&self, route: String, body: S) -> impl Future<Item=T, Error=Error>
        where S: Serialize + Send,
        T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.0.post(url.as_str()).json(&body).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    pub fn patch<S, T>(&self, route: String, body: S) -> impl Future<Item=T, Error=Error>
        where S: Serialize + Send,
        T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.0.patch(url.as_str()).json(&body).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    pub fn put<S, T>(&self, route: String, body: S) -> impl Future<Item=T, Error=Error>
        where S: Serialize + Send,
        T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.0.put(url.as_str()).json(&body).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }

    pub fn delete<T>(&self, route: String) -> impl Future<Item=T, Error=Error>
        where T: DeserializeOwned + Send + 'static,
    {
        let url = format!("{}{}", BASE_URL, route);
        self.0.delete(url.as_str()).send()
            .and_then(|mut res| res.json::<T>())
            .map_err(Error::from)
    }
}