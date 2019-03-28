//! A rich Spectacles Gateway client for Rust.
//!
//! ## Features
//! - Asynchronous websocket message handling.
//! - Zero-Downtime shard spawning.
//! - Integrates seamlessly with the spectacles-brokers package.
//!
//! ## About
//! This crate allows you to interact with the Discord gateway. Please refer to the [Discord Gateway Docs](https://discordapp.com/developers/docs/topics/gateway) for more background on how to use this message.
//!
//! ## Example
//! ```rust,norun
//! #[macro_use] extern crate log;
//! use std::env::var;
//! use tokio::runtime::current_thread;
//! use spectacles_gateway::{ShardManager, ShardStrategy, ManagerOptions, EventHandler, Shard};
//! use spectacles_model::gateway::ReceivePacket;
//! use futures::future::Future;
//!
//! fn main() {
//!     env_logger::init();
//!     let token = var("DISCORD_TOKEN").expect("No Discord Token was provided.");
//!     // Here, we bootstrap our application.
//!     current_thread::run({
//!         // calling new() here return a new instance of the shard manager.
//!         ShardManager::new(token, ManagerOptions {
//!             strategy: ShardStrategy::Recommended,
//!             handler: Handler
//!         })
//!         .map(|manager| manager.begin_spawn()) // Begins spawning of shards.
//!         .map_err(|err| error!("An error occurred while processing shards: {:?}", err))
//!     });
//! }
//! /// Here we define our Handler struct, which we implement the EventHandler trait for.
//! /// The on_packet() trait method will be called when a packet is received from the Discord gateway.
//! struct Handler;
//! impl EventHandler for Handler {
//!      fn on_packet(&self, shard: &mut Shard, pkt: ReceivePacket) {
//!          println!("Received Gateway Packet from Shard {:?} - {:?}", shard.info, pkt);
//!          // Do other things with message, such as sending it to a message broker.
//!      }
//!  }
//! ```

#![feature(futures_api, async_await, await_macro)]
#[macro_use] extern crate log;
#[macro_use]
extern crate tokio;

#[warn(rust_2018_idioms)]

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
    /// Executed when a shard emits a ready event.
    fn on_shard_ready(&self, _shard: &mut Shard);
    /// Executed whenever a raw packet is received.
    fn on_packet(&self, _shard: &mut Shard, _pkt: ReceivePacket);
}
