use std::{
    collections::HashMap,
    sync::Arc,
};

use futures::future::{Future, join_all};
use parking_lot::{Mutex, RwLock};

use spectacles_model::gateway::GatewayBot;

use crate::{
    constants::API_BASE,
    errors::Error,
    shard::Shard
};

pub type ShardMap = HashMap<i32, Arc<Mutex<Shard>>>;
/// An organized group of Discord gateway shards.
pub struct Cluster {
    token: String,
    shards: RwLock<ShardMap>
}

impl Cluster {
    /// Creates a new cluster, with the provided Discord API token.
    pub fn new(token: String) -> Cluster {
        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };
        Self {
            token,
            shards: RwLock::new(HashMap::new()),
        }
    }

    /// Spawns shards up to the specified amount and identifies them with Discord.
    pub fn spawn(self, shards: u64) -> impl Future<Item = ShardMap, Error = Error>  {
        let mut coll = Vec::new();
        info!("[Manager] Attempting to spawn {} shards.", shards);
        for i in 0..shards {
            let token = self.token.clone();
            coll.push(Shard::new(token, [i, shards]));
        }
        join_all(coll).map(move |shards| {
            for (id, shard) in shards.into_iter().enumerate() {
                self.shards.write().insert(id as i32, Arc::new(Mutex::new(shard)));
            };
            for (id, shard) in self.shards.read().iter() {
                info!("[Manager] Attempting to idenitfy Shard {}", id);
                shard.lock().identify().unwrap();
                info!("[Manager] Shard {} has identified with Discord.", id);
            }
            info!("[Manager] Shard spawning complete.");
            self.shards.read().clone()
        }).map_err(Error::from)
    }
    /// Spawn the recommended amount of shards according to Discord for this token.
    pub fn spawn_recommended(self) -> impl Future<Item = ShardMap, Error = Error> {
        self.get_gateway().and_then(|gb| self.spawn(gb.shards))
    }

    fn get_gateway(&self) -> impl Future<Item = GatewayBot, Error = Error> {
        use reqwest::r#async::Client;
        Client::new().get(format!("{}/gateway/bot", API_BASE).as_str())
            .header("Authorization", self.token.clone())
            .send()
            .and_then(|mut resp| resp.json::<GatewayBot>())
            .map(|gb| gb)
            .from_err()
    }

    /*
    /// Spawn a specific range of shards in this process.
    pub fn spawn_range(shards: [u64; 2], total_shards: i32) {

    }
    */

}