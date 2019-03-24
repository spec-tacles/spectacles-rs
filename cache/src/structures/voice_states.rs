use spectacles_model::voice::VoiceState;

use crate::prelude::*;

/// A store for caching Discord voice states.
#[derive(Clone)]
pub struct VoiceStateStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> VoiceStateStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    
    /// Gets a voice state from the cache, by user ID.
    pub fn get(&self, guild_id: impl Into<u64>, id: impl Into<u64>) -> Option<VoiceState> {
        match self.backend.get(format!("VOICE_STATES:{}", guild_id.into()), id.into().to_string()) {
            Ok(r) => serde_json::from_str::<VoiceState>(&r).ok(),
            Err(_) => None
        }
    }

    /// Gets all voice states for the provided user in the cache.
    pub fn get_all(&self, guild_id: impl Into<u64>) -> Result<HashMap<u64, VoiceState>> {
        let results = self.backend.get_all(format!("VOICE_STATES:{}", guild_id.into()))?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<VoiceState>(&val)?);
        };

        Ok(new_map)
    }

    /// Adds a voice state to the cache.
    pub fn add(&self, guild_id: impl Into<u64>, state: VoiceState) -> Result<()> {
        let json = serde_json::to_string(&state)?;

        self.backend.set(format!("VOICE_STATES:{}", guild_id.into()), state.user_id.0, json)
    }

    /// Remove a voice state from the cache.
    pub fn remove(&self, guild_id: impl Into<u64>, state: VoiceState) -> Result<()> {
        self.backend.remove(format!("VOICE_STATES:{}", guild_id.into()), state.user_id.0)
    }
}

/// An non-blocking implementation of the Voice States store, for use with async backends.
#[derive(Clone)]
pub struct VoiceStateStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> VoiceStateStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets a voice state from the cache, by user ID.
    pub fn get(&self, guild_id: impl Into<u64>, id: impl Into<u64>) -> impl Future<Item = Option<VoiceState>, Error = Error> {
        self.backend.get(format!("VOICE_STATES:{}", guild_id.into()), id.into().to_string())
        .then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<VoiceState>(&r).ok()),
            Err(_) => Ok(None)
        })
    }

    /// Gets all voice states for the provided user ID from the cache.
    pub fn get_all(&self, guild_id: impl Into<u64>) -> impl Future<Item = HashMap<u64, VoiceState>, Error = Error> {
        self.backend.get_all(format!("VOICE_STATES:{}", guild_id.into())).map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<VoiceState>(&val).unwrap());
            };

            mapped
        })
    }

    /// Adds a voice state to the cache.
    pub fn add(&self, guild_id: impl Into<u64>, state: VoiceState) -> impl Future<Item = (), Error = Error> {
        let json = serde_json::to_string(&state).expect("Failed to serialize voice state");
        
        self.backend.set(format!("VOICE_STATES:{}", guild_id.into()), state.user_id.0, json)

    }

    /// Removes a voice state from the cache.
    pub fn remove(&self, guild_id: impl Into<u64>, state: VoiceState) -> impl Future<Item = (), Error = Error> {
        self.backend.remove(format!("VOICE_STATES:{}", guild_id.into()), state.user_id.0)
    }
}