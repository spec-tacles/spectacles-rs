#[macro_use] extern crate serde_derive;

use std::fmt::Display;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer};

pub use user::User;

mod user;
pub mod guild;
pub mod gateway;
pub mod presence;
pub mod message;

/// Used to parse JSON strings to an integer.
pub fn parse_snowflake<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}