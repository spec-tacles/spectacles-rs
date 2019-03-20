use std::collections::HashMap;

use crate::errors::Error;
use crate::prelude::*;

/// An interface for caching Discord objects synchronously with Redis.
#[allow(dead_code)]
#[derive(Clone)]
pub struct RedisAsyncBackend {
    /// The underlying Redis connection.
    pub conn: SharedConnection
}

impl AsyncBackend for RedisAsyncBackend {
    fn get_async<T: DeserializeOwned + Send>(&self, coll: &str, id: String) -> Box<Future<Item = Option<T>, Error = Error>> {
        let query = redis::cmd("HGET").arg(coll).arg(id)
            .query_async(self.conn.clone());
        Box::new(
            query.map(|res| {
                let string: String = res.1;
                serde_json::from_str::<T>(string.as_str()).ok()
            }).from_err()
        )
    }

    fn get_all_async<T: DeserializeOwned + Send>(&self, coll: &str) -> Box<Future<Item = HashMap<String, T>, Error = Error>> {
        Box::new(
            redis::cmd("HGETALL")
                .arg(coll)
                .query_async(self.conn.clone())
                .map(|res| {
                    let vals: HashMap<String, String> = res.1;
                    let mut new_map = HashMap::new();
                    for (key, val) in vals {
                        new_map.insert(key, serde_json::from_str::<T>(&val).unwrap());
                    }

                    new_map
                }).from_err()
        )
    }

    fn set_async<T: Serialize>(&self, coll: &str, key: String, value: T) -> Box<Future<Item = (), Error = Error>> {
        let string = serde_json::to_string(&value).unwrap();
        Box::new(
            redis::cmd("HSET")
            .arg(coll)
            .arg(key)
            .arg(string)
            .query_async(self.conn.clone())
            .map(|(_, ())| ())
            .from_err()
        )

    }

    fn remove_async(&self, coll: &str, key: String) -> Box<Future<Item = (), Error = Error>> {
        Box::new(
            redis::cmd("HDEL")
                .arg(coll)
                .arg(key)
                .query_async(self.conn.clone())
                .map(|(_, ())| ())
                .from_err()
        )
    }

}
