[![crates-io-badge]][crates-io-link]
![Downloads](https://img.shields.io/crates/d/spectacles-gateway.svg?style=for-the-badge)
[![docs-badge]][docs-link]

# Spectacles Gateway
A rich Spectacles gateway client for Rust.

## About
This crate allows you to interact with the Discord gateway. Please refer to the [Discord Gateway Docs](https://discordapp.com/developers/docs/topics/gateway) for more background on how to use this crate.
## Features
- Asynchronous websocket message handling.
- Zero-Downtime shard spawning.
- Integrates seamlessly with the spectacles-brokers package.

## Example - Basic Sharder
```rust
use std::env::var;
use spectacles_gateway::{ShardManager, ShardStrategy};
use spectacles_model::gateway::ReceivePacket;
use futures::future::Future;

fn main() {
    let token = var("DISCORD_TOKEN").expect("No Discord Token was provided.");
    // tokio.run() boostraps our Tokio application.
    tokio::run(ShardManager::new(token, ShardStrategy::Recommended)
        .map(|mut manager| {
            // The start_spawn() method returns a tuple of async streams, for handling spawned shards and shard events.
            let (spawner, events) = manager.start_spawn();
            // Now, we poll the streams concurrently in separate threads.
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
            eprintln!("Failed to bootstrap sharding manager. {:?}", err);
        })
    );
}
```

[crates-io-link]: https://crates.io/crates/spectacles-gateway
[crates-io-badge]: https://img.shields.io/crates/v/spectacles-gateway.svg?style=for-the-badge
[docs-link]: https://docs.rs/spectacles-gateway
[docs-badge]: https://img.shields.io/badge/Documentation-docs.rs-red.svg?style=for-the-badge