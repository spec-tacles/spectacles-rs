use spectacles_model::guild::Role;

use crate::prelude::*;

/// A store for caching Discord roles.
#[derive(Clone)]
pub struct RoleStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> RoleStore<T> {
    /// Gets a role from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all roles for the provided user in the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a role to the cache.
    pub fn add(&self) {}

    /// Remove a role from the cache.
    pub fn remove(&self) {}
}

/// An non-blocking implementation of the Role store, for use with async backends.
#[derive(Clone)]
pub struct RoleStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> RoleStoreAsync<T> {
    /// Gets a role from the cache, by user ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all roles for the provided user ID from the cache.
    pub fn get_all(&self, id: impl Into<u64>) {}

    /// Adds a role to the cache.
    pub fn add(&self) {}

    /// Removes a role from the cache.
    pub fn remove(&self) {}
}