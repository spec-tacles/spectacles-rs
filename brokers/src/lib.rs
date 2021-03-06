//! # spectacles-brokers
//!
//! Message brokers which allow for simple communication between Spectacles services.
//!
//! ## Available Brokers
//! - AMQP - An interface to connect to an AMQP-compliant server.

//! ## Example AMQP Publisher
//! ```rust,norun
//! use std::env::var;
//! use std::net::SocketAddr;
//! use futures::future::Future;
//! use spectacles_brokers::AmqpBroker;
//!
//! fn main() {
//!    let addr = var("AMQP_ADDR").expect("No AMQP server address found.");
//!    let addr: SocketAddr = addr.parse();
//!
//!    let connect = AmqpBroker::new(&addr, "test".to_string(), None);
//!    let result = connect.and_then(|broker| {
//!        let json = r#"{"message": "Example Publish."}"#.as_bytes();
//!        broker.publish("HELLO", json.to_vec())
//!    }).map(|_| {
//!        println!("Message publish succeeded, check the other window!");
//!    }).map_err(|err| {
//!        eprintln!("An error was encountered during publish: {}", err);
//!    });
//!
//!    tokio::run(result);
//! }
//! ```
//!
//! More examples can be found in the examples directory on Github.
#[macro_use] extern crate log;

pub use errors::Error;

mod errors;
/// Utilities for interfacing with an AMQP-based message broker.
pub mod amqp;