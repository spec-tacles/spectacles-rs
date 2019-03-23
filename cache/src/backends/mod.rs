use crate::prelude::*;

pub use self::redis_async::*;
pub use self::redis_sync::*;

mod redis_async;
mod redis_sync;

/// Trait for working with an asynchronous storage backend.
pub trait AsyncBackend {
    fn get(&self, coll: impl ToString, id: impl ToString)
           -> Box<Future<Item=String, Error=Error> + Send>;
    fn get_all(&self, coll: impl ToString)
               -> Box<Future<Item=HashMap<String, String>, Error=Error> + Send>;
    fn set(&self, coll: impl ToString, key: impl ToString, value: impl ToString)
           -> Box<Future<Item=(), Error=Error> + Send>;
    fn remove(&self, coll: impl ToString, key: impl ToString)
              -> Box<Future<Item=(), Error=Error> + Send>;
}

/// Trait for working with a synchronous storage backend.
pub trait Backend {
    fn get(&self, coll: impl ToString, id: impl ToString) -> Result<String>;
    fn get_all(&self, coll: impl ToString) -> Result<HashMap<String, String>>;
    fn set(&self, coll: impl ToString, key: impl ToString, value: impl ToString) -> Result<()>;
    fn remove(&self, coll: impl ToString, key: impl ToString) -> Result<()>;
}