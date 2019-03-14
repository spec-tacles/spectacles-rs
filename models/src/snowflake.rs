use std::fmt;
use serde::{de, de::{Visitor, Deserializer}, Deserialize};
use serde::ser::{Serialize, Serializer};

/// Represents a Twitter snowflake used as IDs in various Discord
#[derive(Default, Debug, Clone)]
pub struct Snowflake(pub u64);

impl Into<u64> for Snowflake {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for Snowflake {
    fn from(u: u64) -> Self {
        Snowflake(u)
    }
}


impl fmt::Display for Snowflake {
    fn fmt(&self, fmtter: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtter, "{}", self.0)
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string().as_str())

    }
}

struct SnowflakeVisitor;

impl <'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A snowflake as a string.")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
    {
        let i: u64 = value.parse().map_err(|_| de::Error::invalid_type(de::Unexpected::Str(value), &self))?;
        Ok(Snowflake(i))
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Snowflake, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(SnowflakeVisitor)
    }
}