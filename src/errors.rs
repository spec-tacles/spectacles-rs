use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    result::Result as StdResult,
};

use failure::Fail;
use lapin_futures::error::Error as LapinError;
use reqwest::Error as ReqwestError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Lapin(LapinError),
    Io(IoError),
    Reqwest(ReqwestError)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Lapin(e) => e.name().unwrap(),
            Error::Reqwest(e) => e.description(),
            Error::Io(e) => e.description()
        }
    }
}

impl From<LapinError> for Error {
    fn from(err: LapinError) -> Self {
        Error::Lapin(err)
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