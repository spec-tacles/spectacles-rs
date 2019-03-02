use std::{
    collections::HashMap,
    sync::Arc,
};

use parking_lot::{Mutex, RwLock};

use crate::shard::Shard;

/// An organized group of Discord gateway shards.
pub struct Cluster {
    token: String,
    shard_count: i32,
    shards: RwLock<HashMap<i32, Arc<Mutex<Shard>>>>
}

impl Cluster {
    /// Creates a new cluster, with the provided Discord API token.
    pub fn new(token: String) -> Cluster {
        Self {
            token,
            shards: RwLock::new(HashMap::new()),
            shard_count: -1
        }
    }
    /// Spawns Discord shards according to a provided range.
    /// If no shard count was previously provided, the recommended amount of shards will be spawned.
    pub fn spawn_range(&self, min: i32, max: i32) {

    }

    /// Spawn
    pub fn spawn_recommended(&self) {

    }

    fn _create_shard(&self) {

    }
}