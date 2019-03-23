use spectacles_model::guild::GuildMember;

use crate::prelude::*;

/// A store for caching Discord guild members.
#[derive(Clone)]
pub struct MemberStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> MemberStore<T> {
    /// Gets a guild member from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all guild members for the provided user in the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a guild member to the cache.
    pub fn add(&self) {}

    /// Remove a guild member from the cache.
    pub fn remove(&self) {}
}

/// An non-blocking implementation of the Presence store, for use with async backends.
#[derive(Clone)]
pub struct MemberStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> MemberStoreAsync<T> {
    /// Gets a guild member from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all guild members for the provided user ID from the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a guild member to the cache.
    pub fn add(&self) {}

    /// Removes a guild member from the cache.
    pub fn remove(&self) {}
}