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
//! #![feature(futures_api, async_await, await_macro)]
//! #[macro_use] extern crate tokio;
//! #[macro_use] extern crate log;
//! use tokio::prelude::*;
//! use std::env::var;
//! use spectacles_gateway::{ShardManager, ShardStrategy};

//! fn main() {
//!    env_logger::init();
//!    let token = var("DISCORD_TOKEN").expect("No Discord Token was provided.");
//!    tokio::run_async(async {
//!        let mut manager = await!(ShardManager::new(token, ShardStrategy::Recommended))
//!            .expect("Failed to create shard manager");
//!        let (mut spawner, mut events) = manager.begin_spawn();
//!        tokio::spawn_async(async move {
//!            while let Some(Ok(shard)) = await!(spawner.next()) {
//!                println!("Shard {:?} spawned.", shard.lock().info);
//!            };
//!        });
//!        tokio::spawn_async(async move {
//!            while let Some(Ok(event)) = await!(events.next()) {
//!                if let Some(evt) = event.packet.t {
//!                    println!("Received event from Shard {:?}: {:?}", event.shard.lock().info, evt);
//!                }
//!            };
//!        });
//!    })
//! }
//! ```

#![feature(futures_api, async_await, await_macro)]
#[warn(rust_2018_idioms)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate tokio;

pub use errors::{Error, Result};
pub use manager::*;
pub use shard::Shard;

mod manager;
mod shard;
mod constants;
mod errors;
mod queue;
