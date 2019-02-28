use std::{
    collections::HashMap,
    sync::Arc,
};

use parking_lot::Mutex;

use crate::shard::Shard;

pub struct Cluster {
    token: String,
    shard_count: i32,
    shards: HashMap<i32, Arc<Mutex<Shard>>>
}

impl Cluster {
    /// Creates a new cluster, with the provided Discord API token.
    pub fn new(token: String) -> Cluster {
        Self {
            token,
            shards: HashMap::new(),
            shard_count: -1
        }
    }

    /// Sets the amount of shards to be spawned with this cluster.
    pub fn with_shards(&mut self, amount: i32){
        self.shard_count = amount;
    }

    /// Spawns Discord shards according to the provided shard count.
    /// If no shard count was previously provided, the recommended amount of shards will be spawned,
    pub fn spawn(min: i32, max: i32) {

    }
}