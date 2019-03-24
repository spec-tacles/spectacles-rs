use spectacles_model::guild::Role;

use crate::prelude::*;

/// A store for caching Discord roles.
#[derive(Clone)]
pub struct RoleStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> RoleStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets a role from the cache, by user ID.
    pub fn get(&self, guild_id: impl Into<u64>, id: impl Into<u64>) -> Option<Role> {
        match self.backend.get(format!("ROLES:{}", guild_id.into()), id.into().to_string()) {
            Ok(r) => serde_json::from_str::<Role>(&r).ok(),
            Err(_) => None
        }
    }

    /// Gets all roles for the provided user in the cache.
    pub fn get_all(&self, id: impl Into<u64>) -> Result<HashMap<u64, Role>> {
        let results = self.backend.get_all(format!("ROLES:{}", id.into()))?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<Role>(&val)?);
        };

        Ok(new_map)

    }

    /// Adds a role to the cache.
    pub fn add(&self, guild_id: impl Into<u64>, role: Role) -> Result<()> {
        let json = serde_json::to_string(&role)?;

        self.backend.set(format!("ROLES:{}", guild_id.into()), role.id.0, json)
    }

    /// Remove a role from the cache.
    pub fn remove(&self, guild_id: impl Into<u64>, role: Role) -> Result<()> {
        self.backend.remove(format!("ROLES:{}", guild_id.into()), role.id.0)
    }
}

/// An non-blocking implementation of the Role store, for use with async backends.
#[derive(Clone)]
pub struct RoleStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> RoleStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets a role from the cache, by user ID.
    pub fn get(&self, guild_id: impl Into<u64>, id: impl Into<u64>) -> impl Future<Item = Option<Role>, Error = Error> {
        self.backend.get(format!("ROLES:{}", guild_id.into()), id.into().to_string())
        .then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<Role>(&r).ok()),
            Err(_) => Ok(None)
        })

    }

    /// Gets all roles for the provided user ID from the cache.
    pub fn get_all(&self, guild_id: impl Into<u64>) -> impl Future<Item = HashMap<u64, Role>, Error = Error> {
        self.backend.get_all(format!("ROLES:{}", guild_id.into())).map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<Role>(&val).unwrap());
            };

            mapped
        })

    }

    /// Adds a role to the cache.
    pub fn add(&self, guild_id: impl Into<u64>, role: Role) -> impl Future<Item = (), Error = Error> {
        let json = serde_json::to_string(&role).expect("Failed to serialize role");
        
        self.backend.set(format!("ROLES:{}", guild_id.into()), role.id.0, json)

    }

    /// Removes a role from the cache.
    pub fn remove(&self, guild_id: impl Into<u64>, role: Role) -> impl Future<Item = (), Error = Error> {
        self.backend.remove(format!("ROLES:{}", guild_id.into()), role.id.0)
        
    }
}