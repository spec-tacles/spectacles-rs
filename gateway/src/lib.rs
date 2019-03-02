//! This library provides an interface to the Spectacles Gateway.

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;

pub use cluster::Cluster;
pub use manager::GatewayManager;
pub use shard::Shard;

mod cluster;
mod shard;
mod manager;
mod constants;
mod errors;

