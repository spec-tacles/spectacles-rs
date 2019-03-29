use spectacles_model::guild::Guild;

use crate::backends::Backend;
use crate::prelude::*;

/// A store for caching Discord guilds.
#[derive(Clone)]
pub struct GuildStore<T: Backend> {
    pub backend: T
}

impl<T: Backend> GuildStore<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    /// Gets a guild from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) -> Option<Guild> {
        let res = self.backend.get("GUILDS", id.into().to_string()).unwrap();
        serde_json::from_str::<Guild>(&res).ok()
    }

    /// Gets all guilds in the cache.
    pub fn get_all(&self) -> Result<HashMap<u64, Guild>> {
        let results = self.backend.get_all("GUILDS")?;
        let mut new_map = HashMap::new();
        for (key, val) in results {
            new_map.insert(key.parse::<u64>()?, serde_json::from_str::<Guild>(&val)?);
        };

        Ok(new_map)
    }

    /// Adds a guild to the cache.
    pub fn add(&self, mut entity: Guild) -> Result<()> {
        for member in entity.members {
            let member_str = serde_json::to_string(&member)?;
            self.backend.set(format!("MEMBERS:{}", &entity.id), &member.user.unwrap().id, member_str)?;
        };
        for role in entity.roles {
            let role_str = serde_json::to_string(&role)?;
            self.backend.set(format!("ROLES:{}", &entity.id), role.id.0, role_str)?;
        };
        for emoji in entity.emojis {
            let emoji_str = serde_json::to_string(&emoji)?;
            self.backend.set("EMOJIS", emoji.id.expect("Invalid Emoji ID provided").0, emoji_str)?;
        };
        for voice_state in entity.voice_states {
            let voice_str = serde_json::to_string(&voice_state)?;
            self.backend.set(format!("VOICE_STATES:{}", &entity.id), voice_state.user_id.0, voice_str)?;
        };
        for channel in entity.channels {
            let channel_str = serde_json::to_string(&channel)?;
            self.backend.set("CHANNELS", channel.id.0, channel_str)?;
        };
        for presence in entity.presences.unwrap() {
            let presence_str = serde_json::to_string(&presence)?;
            self.backend.set("PRESENCES", presence.user.id.0, presence_str)?;
        }

        entity.channels = vec![];
        entity.emojis = vec![];
        entity.roles = vec![];
        entity.voice_states = vec![];
        entity.presences = None;
        entity.members = vec![];

        let json = serde_json::to_string(&entity)?;
        self.backend.set("GUILDS", entity.id.clone(), json)
    }


    /// Removes a guild from the cache.
    pub fn remove(&self, entity: Guild) -> Result<()> {
        self.backend.remove("GUILDS", entity.id)
    }

    /// Calculates the total amount of presences in the cache.
    pub fn size(&self) -> Result<u64> {
        self.backend.size("GUILDS")
    }
}


/// A non-blocking implementation of the Guild store, for use with async backends.
#[derive(Clone)]
pub struct GuildStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> GuildStoreAsync<T> {
    /// Creates a new store instance.
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    
    /// Gets a guild from the cache, by ID.
    pub fn get(self, id: impl Into<u64>) -> impl Future<Item=Option<Guild>, Error=Error> {
        self.backend.get("GUILDS", id.into().to_string()).then(|res| match res {
            Ok(r) => Ok(serde_json::from_str::<Guild>(&r).ok()),
            Err(_) => Ok(None)
        })
    }

    /// Gets all guilds from the cache.
    pub fn get_all(&self) -> impl Future<Item=HashMap<u64, Guild>, Error=Error> {
        self.backend.get_all("GUILDS").map(|results| {
            let mut mapped = HashMap::new();
            for (key, val) in results {
                mapped.insert(key.parse::<u64>().unwrap(), serde_json::from_str::<Guild>(&val).unwrap());
            };

            mapped
        })
    }


    /// Adds a guild to the cache.
    pub fn add(&self, mut entity: Guild) -> impl Future<Item=(), Error=Error> {
        for member in entity.members {
            let member_str = serde_json::to_string(&member).unwrap();
            tokio::spawn(self.backend.set(format!("MEMBERS:{}", &entity.id), &member.user.unwrap().id, member_str)
                .map_err(|err| {
                    error!("Failed to inser guild member into cache. {:?}", err);
                })
            );
        };

        for role in entity.roles {
            let role_str = serde_json::to_string(&role).unwrap();
            tokio::spawn(self.backend.set(format!("ROLES:{}", &entity.id), role.id.0, role_str)
                .map_err(|err| {
                    error!("Failed to insert role into cache. {:?}", err);
                })
            );
        };

        for emoji in entity.emojis {
            let emoji_str = serde_json::to_string(&emoji).unwrap();
            tokio::spawn(self.backend.set(format!("ROLES:{}", &entity.id), emoji.id.unwrap().0, emoji_str)
                .map_err(|err| {
                    error!("Failed to insert emoji into cache. {:?}", err);
                })
            );
        };

        for voice_state in entity.voice_states {
            let voice_str = serde_json::to_string(&voice_state).unwrap();
            tokio::spawn(self.backend.set(format!("VOICE_STATES:{}", &entity.id), voice_state.user_id.0, voice_str)
                .map_err(|err| {
                    error!("Failed to insert voice state into cache. {:?}", err);
                })
            );
        };
        for channel in entity.channels {
            let channel_str = serde_json::to_string(&channel).unwrap();
            tokio::spawn(self.backend.set("CHANNELS", channel.id.0, channel_str)
                .map_err(|err| {
                    error!("Failed to insert channel into cache. {:?}", err);
                })
            );
        };

        for presence in entity.presences.unwrap() {
            let presence_str = serde_json::to_string(&presence).unwrap();
            tokio::spawn(self.backend.set("PRESENCES", presence.user.id.0, presence_str)
                .map_err(|err| {
                    error!("Failed to insert presence into cache. {:?}", err);
                })
            );
        }

        entity.channels = vec![];
        entity.emojis = vec![];
        entity.roles = vec![];
        entity.voice_states = vec![];
        entity.presences = None;
        entity.members = vec![];

        let json = serde_json::to_string(&entity).unwrap();
        self.backend.set("GUILDS", entity.id.clone(), json)
    }

    /// Removes a guild from the cache.
    pub fn remove(&self, entity: Guild) -> impl Future<Item=(), Error=Error> {
        self.backend.remove("GUILDS", entity.id)
    }

    /// Calculates the total amount of guilds in the cache.
    pub fn size(&self) -> impl Future<Item=u64, Error=Error> {
        self.backend.size("GUILDS")
    }
}