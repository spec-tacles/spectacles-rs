//! # spectacles-brokers
//!
//! A collection of message brokers which allow for simple communication between microservices.
//!
//! ## About
//! With a message broker, you can unify all of your microservices, and communicate seamlessly, across servers.
//!
//! ## Available Brokers
//! - AMQP - An interface to connect to an AMQP-compliant server.
//!
//! See each broker folder to learn more.

//! ```rust,norun
//! #![feature(futures_api, async_await, await_macro)]
//! #[macro_use] extern crate tokio;
//! 
//! use std::env::var;
//! use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};
//!
//! fn main() {
//!     tokio::run_async(async {
//!         let addr = var("AMQP_URL").expect("No AMQP server address found");
//!         let broker = await!(AmqpBroker::new(&addr, "MYGROUP".to_string(), None))
//!             .expect("Failed to connect to broker");
//!         let json = b"{'message': 'A MESSAGE HERE'}";
//!
//!         match await!(broker.publish("MYQUEUE", json.to_vec(), properties)) {
//!             Ok(_) => println!("{} Messages published.", publish_count),
//!             Err(e) => eprintln!("An error was encountered during publish: {}", e)
//!         }
//!     }
//! }
//! ```
//!
//! More examples can be found in the examples directory on Github.
#![feature(async_await, await_macro, futures_api)]
#[macro_use] extern crate log;
#[macro_use]
extern crate tokio;

pub use errors::Error;

mod errors;
/// Utilities for interfacing with an AMQP-based message broker.
pub mod amqp;

