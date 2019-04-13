//! # Spectacles Gateway
//! This library provides an interface for spawning Discord shards with the Discord Gateway.
//! ## Getting Started
//! This crate uses non-blocking, asynchronous I/O provided with the tokio runtime.
//!
//! To begin spawning shards, simply create a new [`ShardManager`], and choose a [`ShardStrategy`].
//! ```rust, norun
//! use spectacles_gateway::{ShardManager, ShardStrategy};
//! use std::env::var;
//! use tokio::prelude::*;
//!
//! fn main() {
//!     let token = var("DISCORD_TOKEN").expect("Failed to parse Discord token");
//!     // Creating a shard manager
//!     tokio::run(ShardManager::new(token, ShardStrategy::Recommended)
//!         .map(|mut manager| {
//!             // Here we obtain our two streams, responsible for spawned shards and events.
//!             let (spawner, events) = manager.start_spawn();
//!             // We poll each stream concurrently in separate threads.
//!             tokio::spawn(spawner.for_each(|shard| { // Freshly spawned shard
//!                 println!("Shard {:?} has successfully spawned.", shard.lock().info);
//!
//!                 Ok(())
//!             }));
//!             tokio::spawn(events.for_each(|event| { // Event, which contains the shard it belongs to, as well as the Discord packet
//!                 if let Some(evt) = event.packet.t {
//!                      println!("Received event from Shard {:?}: {:?}", event.shard.lock().info, evt);
//!                  };
//!
//!                  Ok(())
//!             }));
//!         })
//!         .map_err(|err| {
//!             eprintln!("Failed to bootstrap sharding manager. {:?}", err);
//!         })
//!     );
//! }
//! ```
//!
//! [`ShardManager`]: struct.ShardManager.html
//! [`ShardStrategy`]: struct.ShardStrategy.html
//!

#[macro_use] extern crate log;

#[warn(rust_2018_idioms)]

pub use errors::{Error, Result};
pub use manager::*;
pub use shard::Shard;

mod manager;
mod shard;
mod constants;
mod errors;
mod queue;
