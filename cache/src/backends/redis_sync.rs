use std::sync::Arc;

use redis::Connection as RedisConnection;

use crate::prelude::*;

/// An interface for caching Discord objects with Redis.
#[allow(dead_code)]
#[derive(Clone)]
pub struct RedisBackend {
    /// The underlying Redis connection.
    pub conn: Arc<RedisConnection>
}

impl Backend for RedisBackend {
    fn get<T>(&self, coll: &str, id: String) -> Option<T>
        where T: DeserializeOwned + Send + 'static
    {
        let res = redis::cmd("HGET").arg(coll).arg(coll).arg(id).query::<String>(&*self.conn).ok();

        match res {
            Some(ref r) => serde_json::from_str::<T>(r).ok(),
            None => None
        }
    }

    fn get_all<T>(&self, coll: &str) -> Result<HashMap<String, T>>
        where T: DeserializeOwned + Send + 'static
    {
        let map: HashMap<String, String> = redis::cmd("HGETALL").arg(coll).query(&*self.conn)?;
        let mut new_map = HashMap::new();
        for (key, val) in map {
            new_map.insert(key, serde_json::from_str::<T>(&val)?);
        }

        Ok(new_map)
    }

    fn set<T: Serialize>(&self, coll: &str, key: String, value: T) -> Result<()> {
        let string = serde_json::to_string(&value).unwrap();
        let _: () = redis::cmd("HSET").arg(coll).arg(key).arg(string).query(&*self.conn)?;

        Ok(())
    }

    fn remove(&self, coll: &str, key: String) -> Result<()> {
        let _: () = redis::cmd("HDEL").arg(coll).arg(key).query(&*self.conn)?;

        Ok(())
    }
}