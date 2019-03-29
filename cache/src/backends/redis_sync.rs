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
    fn get(&self, coll: impl ToString, id: impl ToString) -> Result<String>
    {
        redis::cmd("HGET")
            .arg(coll.to_string())
            .arg(id.to_string())
            .query::<String>(&*self.conn)
            .map_err(Error::from)
    }

    fn get_all(&self, coll: impl ToString) -> Result<HashMap<String, String>> {
        let map: HashMap<String, String> = redis::cmd("HGETALL").arg(coll.to_string()).query(&*self.conn)?;

        Ok(map)
    }

    fn set(&self, coll: impl ToString, key: impl ToString, value: impl ToString) -> Result<()> {
        let _: () = redis::cmd("HSET")
            .arg(coll.to_string())
            .arg(key.to_string())
            .arg(value.to_string())
            .query(&*self.conn)?;

        Ok(())
    }

    fn remove(&self, coll: impl ToString, key: impl ToString) -> Result<()> {
        let _: () = redis::cmd("HDEL")
            .arg(coll.to_string())
            .arg(key.to_string())
            .query(&*self.conn)?;

        Ok(())
    }

    fn size(&self, coll: impl ToString) -> Result<u64> {
        redis::cmd("HGET")
            .arg(coll.to_string())
            .query::<u64>(&*self.conn)
            .map_err(Error::from)
    }
}

impl RedisBackend {
    /// Creates an Redis backend with the provided connection.
    pub fn new(conn: RedisConnection) -> Self {
        Self { conn: Arc::new(conn) }
    }
}