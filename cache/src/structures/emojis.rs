use spectacles_model::message::Emoji;

use crate::prelude::*;

/// A store for caching Discord emojis.
#[derive(Clone)]
pub struct EmojiStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> EmojiStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets an emoji from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) -> Option<Emoji> {
        match self.backend.get("EMOJIS", id.into().to_string()) {
            Ok(r) => serde_json::from_str::<Emoji>(&r).ok(),
            Err(_) => None
        }
    }

    /// Gets all of the emojis from the cache.
    pub fn get_all(&self) -> Result<HashMap<u64, Emoji>> {
        let results = self.backend.get_all("EMOJIS")?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<Emoji>(&val)?);
        };

        Ok(new_map)
    }

    /// Adds an emoji to the cache, by ID.
    pub fn add(&self, emoji: Emoji) -> Result<()> {
        let json = serde_json::to_string(&emoji)?;
        self.backend.set("EMOJIS", emoji.id.expect("Invalid Emoji ID").0, json)
    }

    /// Removes an emoji from the cache, by ID.
    pub fn remove(&self, emoji: Emoji) -> Result<()> {
        self.backend.remove("EMOJIS", emoji.id.expect("Invalid Emoji ID").0)
    }
}

/// A non-blocking implementation of the Emoji store, for use with async backends.
#[derive(Clone)]
pub struct EmojiStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> EmojiStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    
    /// Gets an emoji from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) -> impl Future<Item = Option<Emoji>, Error = Error> {
        self.backend.get("EMOJIS", id.into().to_string()).then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<Emoji>(&r).ok()),
            Err(_) => Ok(None)
        })
    }

    /// Gets all emojis from the cache.
    pub fn get_all(&self) -> impl Future<Item = HashMap<u64, Emoji>, Error = Error> {
        self.backend.get_all("EMOJIS").map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<Emoji>(&val).unwrap());
            };

            mapped
        })
    }

    /// Adds an emoji to the cache.
    pub fn add(&self, emoji: Emoji) -> impl Future<Item = (), Error = Error> {
        let json = serde_json::to_string(&emoji).unwrap();
        self.backend.set("EMOJIS", emoji.id.expect("Invalid Emoji ID provided").0, json)

    }

    /// Removes an emoji from the cache.
    pub fn remove(&self, emoji: Emoji) -> impl Future<Item = (), Error = Error> {
        self.backend.remove("EMOJIS", emoji.id.expect("Invalid Emoji ID provided").0)
    }
}