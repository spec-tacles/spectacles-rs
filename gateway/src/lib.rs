//! This library provides an interface to the Spectacles Gateway.

#[macro_use] extern crate log;

/// Spectacles gateway client utilities.
pub mod client;
/// An organized group of shards.
pub mod cluster;
/// A Spectacles Gateway Shard.
pub mod shard;

mod constants;
mod errors;