pub use backends::*;
pub use errors::Error;

use crate::backends::{AsyncBackend, Backend};
use crate::prelude::*;

mod backends;
mod errors;
mod prelude;

/// The main cache client.
#[derive(Clone)]
pub struct CacheClient<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl <T: Backend> CacheClient <T>
    where T: Backend
{
    /// Creates a new caching layer using an asynchronous Redis connection.
    #[allow(dead_code)]
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    /// Synchronously get an item from the cache.
    pub fn get<C>(self, coll: &str, id: String) -> Option<C>
        where C: DeserializeOwned + Send + 'static
    {
        self.backend.get(coll, id)
    }

    /// Synchronously get all items from the cache.
    pub fn get_all<C>(self, coll: &str) -> Result<HashMap<String, C>>
        where C: DeserializeOwned + Send + 'static
    {
        self.backend.get_all(coll)
    }
}

/// An asynchronous cache client for working with backends.
#[derive(Clone)]
pub struct CacheClientAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl <T: AsyncBackend> CacheClientAsync <T> {
    #[allow(dead_code)]
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    /// Asynchronously gets an item from the cache.
    pub fn get<C>(self, coll: &str, id: String) -> Box<Future<Item = Option<C>, Error = Error>>
        where C: DeserializeOwned + Send
    {
        self.backend.get_async(coll, id)
    }

    /// Asynchronously get all items from the cache.
    pub fn get_all<C>(self, coll: &str) -> Box<Future<Item = HashMap<String, C>, Error = Error>>
        where C: DeserializeOwned + Send
    {
        self.backend.get_all_async(coll)
    }


    /// Asynchronously sets an item into the cache.
    pub fn set<C: Serialize>(self, coll: &str, key: String, value: C) -> Box<Future<Item = (), Error = Error>> {
        self.backend.set_async(coll, key, value)
    }

    /// Asynchronously remove an item from the cache.
    pub fn remove_async(self, coll: &str, id: String) -> Box<Future<Item = (), Error = Error>> {
        self.backend.remove_async(coll, id)
    }
}

