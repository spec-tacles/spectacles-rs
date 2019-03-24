#[macro_use]
extern crate log;

pub use errors::Error;

use crate::{
    backends::{AsyncBackend, Backend},
    structures::{
        channels::*,
        emojis::*,
        guilds::*,
        members::*,
        presences::*,
        roles::*,
        voice_states::*,
    },
};

/// At set of included storage backends.
pub mod backends;
/// A collection of included stores for caching Discord objects.
pub mod structures;
mod errors;
mod prelude;

/// The main cache client.
#[derive(Clone)]
pub struct CacheClient<T: Backend + Clone> {
    /// A store for caching Discord channels.
    pub channels: ChannelStore<T>,
    /// A store for caching Discord emojis.
    pub emojis: EmojiStore<T>,
    /// A store for caching Discord guilds.
    pub guilds: GuildStore<T>,
    /// A store for caching Discord guild members.
    pub members: MemberStore<T>,
    /// A storre for caching Discord presences.
    pub presences: PresenceStore<T>,
    /// A store for caching Discord roles.
    pub roles: RoleStore<T>,
    /// A store for caching Discord voice states.
    pub voice_states: VoiceStateStore<T>,
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend + Clone> CacheClient<T> {
    /// Creates a new synchronous client with the provided backend.
    pub fn new(backend: T) -> Self {
        CacheClient {
            backend: backend.clone(),
            channels: ChannelStore::new(backend.clone()),
            guilds: GuildStore::new(backend.clone()),
            emojis: EmojiStore::new(backend.clone()),
            members: MemberStore::new(backend.clone()),
            presences: PresenceStore::new(backend.clone()),
            roles: RoleStore::new(backend.clone()),
            voice_states: VoiceStateStore::new(backend.clone())
        }
    }
}

/// An asynchronous cache client for working with backends.
#[derive(Clone)]
pub struct CacheClientAsync<T: AsyncBackend + Send + Clone> {
    /// A store for caching Discord channels.
    pub channels: ChannelStoreAsync<T>,
    /// A store for caching Discord guilds.
    pub guilds: GuildStoreAsync<T>,
    /// A store for caching Discord emojis.
    pub emojis: EmojiStoreAsync<T>,
    /// A store for caching Discord guild members.
    pub members: MemberStoreAsync<T>,
    /// A store for caching Discord presences.
    pub presences: PresenceStoreAsync<T>,
    /// A store for caching Discord roles.
    pub roles: RoleStoreAsync<T>,
    /// A store for caching Discord voice states.
    pub voice_states: VoiceStateStoreAsync<T>,
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend + Send + Clone> CacheClientAsync<T> {
    /// Creates a new asynchronous client with the provided backend.
    pub fn new(backend: T) -> Self {
        CacheClientAsync {
            backend: backend.clone(),
            channels: ChannelStoreAsync::new(backend.clone()),
            guilds: GuildStoreAsync::new(backend.clone()),
            emojis: EmojiStoreAsync::new(backend.clone()),
            members: MemberStoreAsync::new(backend.clone()),
            presences: PresenceStoreAsync::new(backend.clone()),
            roles: RoleStoreAsync::new(backend.clone()),
            voice_states: VoiceStateStoreAsync::new(backend.clone())
        }
    }
}


