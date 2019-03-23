use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    result::Result as StdResult
};

use redis::RedisError;
use serde_json::Error as JsonError;

pub type Result<T> = StdResult<T, Error>;

/// Various errors that may be encountered while using the crate.
#[derive(Debug)]
pub enum Error {
    Redis(RedisError),
    ParseInt(ParseIntError),
    Json(JsonError)
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}


impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Redis(e) => e.description(),
            Error::ParseInt(e) => e.description(),
            Error::Json(e) => e.description()
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl From<RedisError> for Error {
    fn from(err: RedisError) -> Self {
        Error::Redis(err)
    }
}


impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}