# spectacles-gateway

A rich Spectacles Gateway client for Rust.

##Features
- Asynchronous websocket message handling.
- Packets deserialized with ETF (Erlang Time Format) for maximum efficiency
- Zero-Downtime shard spawning.


## Example
```rust,norun
#[macro_use] extern crate log;
use std::env::var;
use spectacles_gateway::{ShardManager, ShardStrategy, ManagerOptions, EventHandler, Shard};
use spectacles_model::gateway::ReceivePacket;
use futures::future::Future;

fn main() {
    env_logger::init();
    let token = var("DISCORD_TOKEN").expect("No Discord Token was provided.");
    tokio::run({
        /// calling new () here return a new instance of the shard manager.
        ShardManager::new(token, ManagerOptions {
            strategy: ShardStrategy::Recommended,
            handler: Handler
        }).and_then(|mut manager| manager.spawn()) /// Begins spawning of shards.
            .map_err(|err| error!("An error occured while processing shards: {:?}", err))
    });
}
/// Here we define our Handler struct, which we implement the Eventhandler trait for.
/// The on_packet() trait method will be called when a packet is received from the Discord gateway.
struct Handler;
 impl EventHandler for Handler {
     fn on_packet(&self, shard: Shard, pkt: ReceivePacket) -> Box<Future<Item = (), Error = ()> + Send> {
         println!("Received Gateway Packet from Shard {:?} - {:?}", shard.info, pkt);
         // Do other things with message, such as sending it to a message broker.
         Box::new(futures::future::ok(()))
     }
 }
```