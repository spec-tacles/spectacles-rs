//! A collection of data types for working with various Spectacles modules.

#[macro_use] extern crate serde_derive;

pub use snowflake::*;
pub use user::User;

mod user;
pub mod guild;
pub mod channel;
pub mod voice;
pub mod invite;
pub mod gateway;
pub mod presence;
pub mod message;
pub mod snowflake;