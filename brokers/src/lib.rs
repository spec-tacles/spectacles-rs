//! This package provides a rich event-publishing API which allows for seamless communication between Spectacles services.
#[macro_use] extern crate log;

pub use amqp::AmqpBroker;
pub use errors::Error;

mod errors;
mod amqp;

/// Event handler for receiving messages from a message brokers.
pub trait MessageHandler {
    fn on_message(&self, event: &str, payload: String);
}