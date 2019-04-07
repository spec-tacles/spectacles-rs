use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    num::ParseIntError,
    result::Result as StdResult,
};

use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use tokio::timer::Error as TimerError;

/// A modified result type which encompasses the global error type.
pub type Result<T> = StdResult<T, Error>;

/// Represents a global error which can occur throughout the library.
#[derive(Debug)]
pub enum Error {
    Json(JsonError),
    ParseInt(ParseIntError),
    Timer(TimerError),
    Reqwest(ReqwestError),
    InvalidTokenError,
    Io(IoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Reqwest(e) => e.description(),
            Error::ParseInt(e) => e.description(),
            Error::Timer(e) => e.description(),
            Error::Io(e) => e.description(),
            Error::Json(e) => e.description(),
            Error::InvalidTokenError =>
                "The token provided was not accepted by Discord. Please check that your token is correct and try again."
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        if let Some(t) = err.status() {
            match t.as_u16() {
                401 => Error::InvalidTokenError,
                _ => Error::Reqwest(err)
            }
        } else {
            Error::Reqwest(err)
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl From<TimerError> for Error {
    fn from(err: TimerError) -> Self {
        Error::Timer(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}