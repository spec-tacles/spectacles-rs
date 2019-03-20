use crate::prelude::*;

pub use self::redis_async::*;
pub use self::redis_sync::*;

mod redis_async;
mod redis_sync;
/// Trait for working with any asynchronous storage backend.
pub trait AsyncBackend {
    fn get_async<T: DeserializeOwned + Send>(&self, coll: &str, id: String) -> Box<Future<Item = Option<T>, Error = Error>>;
    fn get_all_async<T: DeserializeOwned + Send>(&self, coll: &str) -> Box<Future<Item = HashMap<String, T>, Error = Error>>;
    fn set_async<T: Serialize>(&self, coll: &str, key: String, value: T) -> Box<Future<Item = (), Error = Error>>;
    fn remove_async(&self, coll: &str, key: String) -> Box<Future<Item = (), Error = Error>>;
}

/// Trait for working with a synchronous storage backend.
pub trait Backend {
    fn get<T: DeserializeOwned + Send + 'static>(&self, coll: &str, id: String) -> Option<T>;
    fn get_all<T: DeserializeOwned + Send + 'static>(&self, coll: &str) -> Result<HashMap<String, T>>;
    fn set<T: Serialize>(&self, coll: &str, key: String, value: T) -> Result<()>;
    fn remove(&self, coll: &str, key: String) -> Result<()>;
}