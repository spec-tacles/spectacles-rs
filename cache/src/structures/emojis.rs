use spectacles_model::message::Emoji;

use crate::prelude::*;

/// A store for caching Discord emojis.
#[derive(Clone)]
pub struct EmojiStore<T: Backend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: Backend> EmojiStore<T> {
    /// Gets an emoji from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all of the emojis from the cache.
    pub fn get_all(&self) {}

    /// Adds an emoji to the cache.
    pub fn add(&self) {}

    /// Removes an emoji from the cache.
    pub fn remove(&self) {}
}

/// A non-blocking implementation of the Emoji store, for use with async backends.
#[derive(Clone)]
pub struct EmojiStoreAsync<T: AsyncBackend> {
    /// The underlying backend instance.
    pub backend: T
}

impl<T: AsyncBackend> EmojiStoreAsync<T> {
    /// Gets an emoji from the cache, by ID.
    pub fn get(&self, id: impl Into<u64>) {}

    /// Gets all emojis from the cache.
    pub fn get_all(&self) {}

    /// Adds an emoji to the cache.
    pub fn add(&self) {}

    /// Removes an emoji from the cache.
    pub fn remove(&self) {}
}