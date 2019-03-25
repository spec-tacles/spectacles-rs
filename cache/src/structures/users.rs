use spectacles_model::User;

use crate::prelude::*;

/// A store for caching Discord users.
#[derive(Clone)]
pub struct UserStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> UserStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    /// Gets a user object from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) -> Option<User> {
        match self.backend.get("EMOJIS", id.into().to_string()) {
            Ok(r) => serde_json::from_str::<User>(&r).ok(),
            Err(_) => None
        }
    }

    /// Gets all users in the cache.
    pub fn get_all(&self) -> Result<HashMap<u64, User>> {
        let results = self.backend.get_all("USERS")?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<User>(&val)?);
        };

        Ok(new_map)
    }

    /// Adds a user to the cache.
    pub fn add(&self, entity: User) -> Result<()> {
        let string = serde_json::to_string(&entity)?;

        self.backend.set("USERS", entity.id.0, string)
    }

    /// Remove a user from the cache.
    pub fn remove(&self, entity: User) -> Result<()> {
        self.backend.remove("USERS", entity.id.0)
    }
}

/// An non-blocking implementation of the User store, for use with async backends.
#[derive(Clone)]
pub struct UserStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> UserStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets a user object from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) -> impl Future<Item=Option<User>, Error=Error> {
        self.backend.get("USERS", id.into().to_string()).then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<User>(&r).ok()),
            Err(_) => Ok(None)
        })
    }

    /// Gets all users from the cache.
    pub fn get_all(&self) -> impl Future<Item=HashMap<u64, User>, Error=Error> {
        self.backend.get_all("USERS").map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<User>(&val).unwrap());
            };

            mapped
        })
    }

    /// Adds a user to the cache.
    pub fn add(&self, chan: User) -> impl Future<Item=(), Error=Error> {
        let json = serde_json::to_string(&chan).unwrap();
        self.backend.set("USERS", chan.id.0, json)
    }

    /// Removes a user from the cache.
    pub fn remove(&self, chan: User) -> impl Future<Item=(), Error=Error> {
        self.backend.remove("USERS", chan.id.0)
    }
}