use spectacles_model::voice::VoiceState;

use crate::prelude::*;

/// A store for caching Discord voice states.
#[derive(Clone)]
pub struct VoiceStateStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> VoiceStateStore<T> {
    /// Gets a voice state from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all voice states for the provided user in the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a voice state to the cache.
    pub fn add(&self) {}

    /// Remove a voice state from the cache.
    pub fn remove(&self) {}
}

/// An non-blocking implementation of the Voice States store, for use with async backends.
#[derive(Clone)]
pub struct VoiceStateStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> VoiceStateStoreAsync<T> {
    /// Gets a voice state from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all voice states for the provided user ID from the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a voice state to the cache.
    pub fn add(&self) {}

    /// Removes a voice state from the cache.
    pub fn remove(&self) {}
}