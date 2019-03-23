#[macro_use]
extern crate log;

pub use errors::Error;

use crate::{
    backends::{AsyncBackend, Backend},
    structures::{
        channels::*,
        emojis::*,
        guilds::*,
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
    /// A store for caching Discord guilds.
    pub guilds: GuildStore<T>,
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend + Clone> CacheClient<T> {
    /// Creates a new synchronous client with the provided backend.
    pub fn new(backend: T) -> Self {
        CacheClient {
            backend: backend.clone(),
            guilds: GuildStore { backend },
        }
    }
}

/// An asynchronous cache client for working with backends.
#[derive(Clone)]
pub struct CacheClientAsync<T: AsyncBackend + Send + Clone> {
    /// A store for caching Discord guilds.
    pub guilds: GuildStoreAsync<T>,
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend + Send + Clone> CacheClientAsync<T> {
    /// Creates a new asynchronous client with the provided backend.
    pub fn new(backend: T) -> Self {
        CacheClientAsync {
            backend: backend.clone(),
            guilds: GuildStoreAsync { backend },
        }
    }
}


