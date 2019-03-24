use spectacles_model::channel::Channel;

use crate::prelude::*;

/// A store for caching Discord channels.
#[derive(Clone)]
pub struct ChannelStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> ChannelStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    /// Gets a channel object from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) -> Option<Channel> {
         match self.backend.get("EMOJIS", id.into().to_string()) {
            Ok(r) => serde_json::from_str::<Channel>(&r).ok(),
            Err(_) => None
        }
    }

    /// Gets all channels in the cache.
    pub fn get_all(&self) -> Result<HashMap<u64, Channel>> {
        let results = self.backend.get_all("CHANNELS")?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<Channel>(&val)?);
        };

        Ok(new_map)
    }

    /// Adds a channel to the cache.
    pub fn add(&self, entity: Channel) -> Result<()> {
        let string = serde_json::to_string(&entity)?;

        self.backend.set("CHANNELS", entity.id.0, string)
    }

    /// Remove a channel from the cache.
    pub fn remove(&self, entity: Channel) -> Result<()> {
        self.backend.remove("CHANNELS", entity.id.0)
    }
}

/// An non-blocking implementation of the Channel store, for use with async backends.
#[derive(Clone)]
pub struct ChannelStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> ChannelStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    
    /// Gets a channel object from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) -> impl Future<Item = Option<Channel>, Error = Error> {
        self.backend.get("CHANNELS", id.into().to_string()).then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<Channel>(&r).ok()),
            Err(_) => Ok(None)
        })
    }

    /// Gets all channels from the cache.
    pub fn get_all(&self) -> impl Future<Item = HashMap<u64, Channel>, Error = Error> {
        self.backend.get_all("CHANNELS").map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<Channel>(&val).unwrap());
            };

            mapped
        })
    }

    /// Adds a channel to the cache.
    pub fn add(&self, chan: Channel) -> impl Future<Item = (), Error = Error> {
        let json = serde_json::to_string(&chan).unwrap();
        self.backend.set("CHANNELS", chan.id.0, json)
    }

    /// Removes a channel from the cache.
    pub fn remove(&self, chan: Channel) -> impl Future<Item = (), Error = Error> {
        self.backend.remove("CHANNELS", chan.id.0)
    }
}