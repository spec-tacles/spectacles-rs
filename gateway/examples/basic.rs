#![feature(futures_api, async_await, await_macro)]
#[macro_use]
extern crate tokio;

use std::env::var;

use tokio::prelude::*;

use spectacles_gateway::{ShardManager, ShardStrategy};

fn main() {
    let token = var("DISCORD_TOKEN").expect("No Discord token provided");
    tokio::run_async(async {
        // Here, we create or shard manager with the provided strategy. If no shard count is provided, then the default strategy will be used.
        let manager_future = {
            // If a shard count environment variable is provided, we use it. Otherwise, we default to the recommended shard count.
            let strategy = match var("SHARD_COUNT") {
                Ok(s) => ShardStrategy::SpawnAmount(s.parse::<usize>().expect("Invalid integer provided")),
                Err(_) => ShardStrategy::Recommended
            };
            ShardManager::new(token, strategy)
        };
        let mut manager = await!(manager_future).expect("Failed to create shard manager");
        let (mut spawner, mut events) = manager.begin_spawn();

        // Now we consume our shard spawning stream in another thread, so we can do other tasks, such as listen for shard events.
        tokio::spawn_async(async move {
            while let Some(Ok(shard)) = await!(spawner.next()) {
                println!("Shard {:?} spawned.", shard.lock().info);
            };
        });

        // As we did above, we poll our shard event stream in a new thread.
        tokio::spawn_async(async move {
            while let Some(Ok(event)) = await!(events.next()) {
                if let Some(evt) = event.t {
                    println!("Received event: {:?}", evt);
                }
            };
        });
    });
}
