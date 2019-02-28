use serde_json::Value;

pub struct Shard {
    id: i32,
    token: String,
    has_acked: bool,
}

impl Shard {
    /// Creates a new Discord Shard, with the provided token.
    pub fn new(token: String) {

    }

    /// Identifies a shard with Discord.
    pub fn identify(&self) {

    }
}