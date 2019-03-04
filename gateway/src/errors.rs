use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    result::Result as StdResult,
};

use futures::sync::mpsc::SendError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use tokio_tungstenite::tungstenite::{
    Error as TungsteniteError,
    Message as TungsteniteMessage
};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Tungstenite(TungsteniteError),
    Json(JsonError),
    Reqwest(ReqwestError),
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
            Error::Tungstenite(e) => e.description(),
            Error::Io(e) => e.description(),
            Error::TungsteniteSend(e) => e.description(),
            Error::Json(e) => e.description(),
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

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
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