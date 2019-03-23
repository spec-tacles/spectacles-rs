use spectacles_model::channel::Channel;

use crate::prelude::*;

/// A store for caching Discord channels.
#[derive(Clone)]
pub struct ChannelStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> ChannelStore<T> {
    /// Gets a channel from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all channels in the cache.
    pub fn get_all(&self) {}

    /// Adds an emoji to the cache.
    pub fn add(&self) {}

    /// Remove a channel from the cache.
    pub fn remove(&self) {}
}

/// An non-blocking implementation of the Channel store, for use with async backends.
#[derive(Clone)]
pub struct ChannelStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> ChannelStoreAsync<T> {
    /// Gets a channel object from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all channels from the cache.
    pub fn get_all(&self) {}

    /// Adds a channel to the cache.
    pub fn add(&self) {}

    /// Removes a channel from the cache.
    pub fn remove(&self) {}
}