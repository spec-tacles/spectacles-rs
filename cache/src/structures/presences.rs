use spectacles_model::presence::Presence;

use crate::prelude::*;

/// A store for caching Discord presences.
pub struct PresenceStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> PresenceStore<T> {
    /// Gets a presence from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all presences for the provided user in the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a presence to the cache.
    pub fn add(&self) {}

    /// Remove a presence from the cache.
    pub fn remove(&self) {}
}

/// An non-blocking implementation of the Presence store, for use with async backends.
pub struct PresenceStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> PresenceStoreAsync<T> {
    /// Gets a presence from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all presences for the provided user ID from the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a presence to the cache.
    pub fn add(&self) {}

    /// Removes a presence from the cache.
    pub fn remove(&self) {}
}