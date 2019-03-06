#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;

pub use errors::{Error, Result};
pub use manager::*;
pub use shard::Shard;
use spectacles_model::gateway::ReceivePacket;

mod manager;
mod shard;
mod constants;
mod errors;
mod queue;

/// Options for Creating a new shard manager.
pub struct ManagerOptions<H>
where H: EventHandler + Send + Sync
{
    /// The struct which contains callbacks for websocket packets.
    pub handler: H,
    /// The strategy in which you want to spawn shards.
    pub strategy: ShardStrategy,
}
/// The event handler trait, useful for receiving events from the websocket.
pub trait EventHandler {
    /// Executed whenever a raw packet is received.
    fn on_packet(&self, _shard: Shard, _pkt: ReceivePacket) -> Box<futures::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }
}
