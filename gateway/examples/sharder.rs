use std::env::var;

use tokio::prelude::*;

use spectacles_gateway::{ShardManager, ShardStrategy};

fn main() {
    let token = var("DISCORD_TOKEN").expect("Failed to parse Discord token");

    // Here, we initialize a new shard manager, with the provided Discord token and strategy.
    // We use the recommended sharding strategy, but a fixed shard strategy may also be chosen.
    tokio::run(ShardManager::new(token, ShardStrategy::Recommended)
        .map(|mut manager| {
            // The start_spawn() method returns a tuple of streams: One for handling spawned shards, the other for handling shard events.
            let (spawner, events) = manager.start_spawn();
            // As demonstrated below, we consume the streams concurrently between threads.
            tokio::spawn(spawner.for_each(|shard| {
                println!("Shard {:?} has successfully spawned.", shard.lock().info);

                Ok(())
            }));
            tokio::spawn(events.for_each(|event| {
                if let Some(evt) = event.packet.t {
                    println!("Received event from Shard {:?}: {:?}", event.shard.lock().info, evt);
                };

                Ok(())
            }));
        })
        .map_err(|err| {
            eprintln!("Error occured: {:?}", err);
        })
    );
}