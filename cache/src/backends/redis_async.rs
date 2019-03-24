use std::collections::HashMap;

use crate::errors::Error;
use crate::prelude::*;

/// An interface for caching Discord objects synchronously with Redis.
#[allow(dead_code)]
#[derive(Clone)]
pub struct RedisBackendAsync {
    /// The underlying Redis connection.
    pub conn: SharedConnection
}

impl AsyncBackend for RedisBackendAsync {    
    fn get(&self, coll: impl ToString, id: impl ToString) -> Box<Future<Item=String, Error=Error> + Send> {
        let query = redis::cmd("HGET")
            .arg(coll.to_string())
            .arg(id.to_string())
            .query_async(self.conn.clone());

        Box::new(
            query.map(|res| {
                let string: String = res.1;
                string
            }).map_err(Error::from)
        )
    }

    fn get_all(&self, coll: impl ToString) -> Box<Future<Item=HashMap<String, String>, Error=Error> + Send> {
        Box::new(
            redis::cmd("HGETALL")
                .arg(coll.to_string())
                .query_async(self.conn.clone())
                .map(|res| {
                    let vals: HashMap<String, String> = res.1;
                    vals
                }).from_err()
        )
    }

    fn set(&self, coll: impl ToString, key: impl ToString, value: impl ToString) -> Box<Future<Item=(), Error=Error> + Send> {
        Box::new(
            redis::cmd("HSET")
                .arg(coll.to_string())
                .arg(key.to_string())
                .arg(value.to_string())
            .query_async(self.conn.clone())
            .map(|(_, ())| ())
            .from_err()
        )

    }

    fn remove(&self, coll: impl ToString, key: impl ToString) -> Box<Future<Item=(), Error=Error> + Send> {
        Box::new(
            redis::cmd("HDEL")
                .arg(coll.to_string())
                .arg(key.to_string())
                .query_async(self.conn.clone())
                .map(|(_, ())| ())
                .from_err()
        )
    }
}

impl RedisBackendAsync {
    /// Creates an asynchronous Redis backend with the provided connection.
    pub fn new(conn: SharedConnection) -> Self {
        Self { conn }
    }
}