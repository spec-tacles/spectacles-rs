use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    result::Result as StdResult,
};

use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Json(JsonError),
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
