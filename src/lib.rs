//! Spectacles is a distributed Discord wrapper.

#[macro_use] extern crate log;

#[cfg(feature = "broker")]
pub mod broker;

mod errors;