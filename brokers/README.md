[![crates-io-badge]][crates-io-link]
![Downloads](https://img.shields.io/crates/d/spectacles-brokers.svg?style=for-the-badge)
[![docs-badge]][docs-link]

# spectacles-brokers

Message brokers which allow for simple communication between Spectacles services.

## Available Brokers
- AMQP - An interface to connect to an AMQP-compliant server.

### Example: Publishing a message

```rust,norun
#![feature(futures_api, async_await, await_macro)]
#[macro_use] extern crate tokio;

use std::env::var;
use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};

fn main() {
    tokio::run_async(async {
        let addr = var("AMQP_URL").expect("No AMQP server address found");
        let broker = await!(AmqpBroker::new(&addr, "MYGROUP".to_string(), None))
            .expect("Failed to connect to broker");
        let json = b"{'message': 'A MESSAGE HERE'}";
        
        match await!(broker.publish("MYQUEUE", json.to_vec(), properties)) {
            Ok(_) => println!("{} Messages published.", publish_count),
            Err(e) => eprintln!("An error was encountered during publish: {}", e)
        }
    }
}
```

More examples can be found in the [`examples`] directory.

[crates-io-link]: https://crates.io/crates/spectacles-brokers
[crates-io-badge]: https://img.shields.io/crates/v/spectacles-brokers.svg?style=for-the-badge
[docs-link]: https://docs.rs/spectacles-brokers
[docs-badge]: https://img.shields.io/badge/Documentation-docs.rs-red.svg?style=for-the-badge