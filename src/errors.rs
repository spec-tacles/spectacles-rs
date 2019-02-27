use lapin_futures::error::Error as LapinError;
use reqwest::Error as ReqwestError;
use failure::Fail;
use std::{
    error::Error as StdError,
    result::Result as StdResult,
    fmt::{Formatter, Result as FmtResult, Display},
};
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Lapin(LapinError),
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
            Error::Reqwest(e) => e.description()
        }
    }
}
