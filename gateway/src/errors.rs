use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    result::Result as StdResult,
};

use futures::sync::mpsc::SendError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use tokio::timer::Error as TimerError;
use tokio_tungstenite::tungstenite::{
    Error as TungsteniteError,
    Message as TungsteniteMessage
};

/// A modified result type which encompasses the global error type.
pub type Result<T> = StdResult<T, Error>;

/// Represents a global error which can occur throughout the library.
#[derive(Debug)]
pub enum Error {
    Tungstenite(TungsteniteError),
    Json(JsonError),
    Reqwest(ReqwestError),
    Timer(TimerError),
    InvalidTokenError,
    Io(IoError),
    TungsteniteSend(SendError<TungsteniteMessage>)
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
            Error::Timer(e) => e.description(),
            Error::Tungstenite(e) => e.description(),
            Error::Io(e) => e.description(),
            Error::TungsteniteSend(e) => e.description(),
            Error::Json(e) => e.description(),
            Error::InvalidTokenError =>
                "The token provided was not accepted by Discord. Please check that your token is correct and try again."
        }
    }
}

impl From<TungsteniteError> for Error {
    fn from(err: TungsteniteError) -> Self {
        Error::Tungstenite(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<TimerError> for Error {
    fn from(err: TimerError) -> Self {
        Error::Timer(err)
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

impl From<SendError<TungsteniteMessage>> for Error {
    fn from(err: SendError<TungsteniteMessage>) -> Self {
        Error::TungsteniteSend(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}
