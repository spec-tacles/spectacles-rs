use spectacles_model::guild::GuildMember;

use crate::prelude::*;

/// A store for caching Discord guild members.
#[derive(Clone)]
pub struct MemberStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> MemberStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets a guild member from the cache, by user ID.
    pub fn get(&self, guild_id: impl Into<u64>, id: impl Into<u64>) -> Option<GuildMember> {
        match self.backend.get(format!("MEMBERS:{}", guild_id.into()), id.into().to_string()) {
            Ok(r) => serde_json::from_str::<GuildMember>(&r).ok(),
            Err(_) => None
        }
    }

    /// Gets all guild members for the provided user in the cache.
    pub fn get_all(&self, guild_id: impl Into<u64>) -> Result<HashMap<u64, GuildMember>> {
        let results = self.backend.get_all(format!("MEMBERS:{}", guild_id.into()))?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<GuildMember>(&val)?);
        };

        Ok(new_map)

    }

    /// Adds a guild member to the cache.
    pub fn add(&self, guild_id: impl Into<u64>, member: GuildMember) -> Result<()> {
        let json = serde_json::to_string(&member)?;
        
        self.backend.set(format!("MEMBERS:{}", guild_id.into()), member.user.expect("No User found for this member.").id.0, json)
    }

    /// Remove a guild member from the cache.
    pub fn remove(&self, guild_id: impl Into<u64>, member: impl Into<u64>) -> Result<()> {
        self.backend.remove(format!("MEMBERS:{}", guild_id.into()), member.into().to_string())
    }

    /// Calculates the total amount of guild members for the provided guild in the cache.
    pub fn size(&self, guild_id: impl Into<u64>) -> Result<u64> {
        self.backend.size(format!("MEMBERS:{}", guild_id.into().to_string()))
    }
}

/// An non-blocking implementation of the Presence store, for use with async backends.
#[derive(Clone)]
pub struct MemberStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> MemberStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    
    /// Gets a guild member from the cache, by user ID.
    pub fn get(&self, guild_id: impl Into<u64>, member: impl Into<u64>) -> impl Future<Item = Option<GuildMember>, Error = Error> 
    {
        self.backend.get(format!("MEMBERS:{}", guild_id.into()), member.into().to_string())
        .then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<GuildMember>(&r).ok()),
            Err(_) => Ok(None)
        })
    }

    /// Gets all guild members for the provided user ID from the cache.
    pub fn get_all(&self, guild_id: impl Into<u64>) -> impl Future<Item = HashMap<u64, GuildMember>, Error = Error> {
        self.backend.get_all(format!("MEMBERS:{}", guild_id.into())).map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<GuildMember>(&val).unwrap());
            };

            mapped
        })

    }

    /// Adds a guild member to the cache.
    pub fn add(&self, guild_id: impl Into<u64>, member: GuildMember) -> impl Future<Item = (), Error = Error> {
        let json = serde_json::to_string(&member).expect("Failed to serialize guild member to string");

        self.backend.set(
            format!("MEMBERS:{}", guild_id.into()), 
            member.user.expect("Invalid guild member ID.").id.0, 
            json
        )
    }

    /// Removes a guild member from the cache.
    pub fn remove(&self, guild_id: impl Into<u64>, member: impl Into<u64>) -> impl Future<Item = (), Error = Error> {
        self.backend.remove(format!("MEMBERS:{}", guild_id.into().to_string()), member.into().to_string())
    }

    /// Calculates the total amount of guild members for the provided guild in the cache.
    pub fn size(&self, guild_id: impl Into<u64>) -> impl Future<Item=u64, Error=Error> {
        self.backend.size(format!("MEMBERS:{}", guild_id.into().to_string()))
    }
}