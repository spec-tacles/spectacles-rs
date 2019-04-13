[![crates-io-badge]][crates-io-link]
![Downloads](https://img.shields.io/crates/d/spectacles-brokers.svg?style=for-the-badge)
[![docs-badge]][docs-link]

# Spectacles Brokers

Message brokers which allow for simple communication between Spectacles services.

## Available Brokers
- AMQP - An interface to connect to an AMQP-compliant server.

## Example: AMQP Publisher
```rust,norun
use std::env::var;
use futures::future::Future;
use spectacles_brokers::amqp::*;

fn main() {
    let addr = var("AMQP_ADDR").expect("No AMQP server address found.");
    let connect = AmqpBroker::new(addr, "test".to_string(), None);
    let result = connect.and_then(|broker| {
        let json = r#"{"message": "Example Publish."}"#.as_bytes();
        let props = AmqpProperties::default().with_content_type("application/json".to_string();
        broker.publish("HELLO", json.to_vec(), props).map_err(|err| {
            eprintln!("An error was encountered during publish: {}", err);
        })
    }).map(|_| {
        println!("Message publish succeeded, check the other window!");
    })

    tokio::run(result);
}

```

More examples can be found in the [`examples`] directory.


[crates-io-link]: https://crates.io/crates/spectacles-brokers
[crates-io-badge]: https://img.shields.io/crates/v/spectacles-brokers.svg?style=for-the-badge
[docs-link]: https://docs.rs/spectacles-brokers
[docs-badge]: https://img.shields.io/badge/Documentation-docs.rs-red.svg?style=for-the-badge