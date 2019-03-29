use spectacles_model::presence::Presence;

use crate::prelude::*;

/// A store for caching Discord presences.
#[derive(Clone)]
pub struct PresenceStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> PresenceStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets a presence from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) -> Option<Presence> {
        match self.backend.get("PRESENCES", id.into().to_string()) {
            Ok(r) => serde_json::from_str::<Presence>(&r).ok(),
            Err(_) => None
        }
    }

    /// Gets all presences for the provided user in the cache.
    pub fn get_all(&self) -> Result<HashMap<u64, Presence>> {
        let results = self.backend.get_all("PRESENCES")?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<Presence>(&val)?);
        };

        Ok(new_map)
    }

    /// Adds a presence to the cache.
    pub fn add(&self, pres: Presence) -> Result<()> {
        let json = serde_json::to_string(&pres)?;

        self.backend.set("PRESENCES", pres.user.id.0, json)
    }

    /// Remove a presence from the cache.
    pub fn remove(&self, pres: Presence) -> Result<()> {
        self.backend.remove("PRESENCES", pres.user.id.0)
    }

    /// Calculates the total amount of presences in the cache.
    pub fn size(&self) -> Result<u64> {
        self.backend.size("PRESENCES")
    }
}

/// An non-blocking implementation of the Presence store, for use with async backends.
#[derive(Clone)]
pub struct PresenceStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> PresenceStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    
    /// Gets a presence from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) -> impl Future<Item = Option<Presence>, Error = Error> {
        self.backend.get("PRESENCES", id.into().to_string())
        .then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<Presence>(&r).ok()),
            Err(_) => Ok(None)
        })
    }

    /// Gets all presences for the provided user ID from the cache.
    pub fn get_all(&self) -> impl Future<Item = HashMap<u64, Presence>, Error = Error> {
        self.backend.get_all("PRESENCES").map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<Presence>(&val).unwrap());
            };

            mapped
        })
    }

    /// Adds a presence to the cache.
    pub fn add(&self, pres: Presence) -> impl Future<Item = (), Error = Error> {
        let json = serde_json::to_string(&pres).expect("Failed to serialize presene structure.");

        self.backend.set("PRESENCES", pres.user.id.0, json)
    }

    /// Removes a presence from the cache.
    pub fn remove(&self, pres: Presence) -> impl Future<Item = (), Error = Error> {
        self.backend.remove("PRESENCES", pres.user.id.0)
    }

    /// Calculates the total amount of presences in the cache.
    pub fn size(&self) -> impl Future<Item=u64, Error=Error> {
        self.backend.size("PRESENCES")
    }
}