//! This package features an event-publishing API which allows for seamless communication between Spectacles services.

#[macro_use] extern crate log;

mod errors;
/// Utilities related to the AMQP message broker.
pub mod amqp;