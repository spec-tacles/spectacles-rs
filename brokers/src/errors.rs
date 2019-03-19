use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

use failure::Fail;
use lapin_futures_native_tls::lapin::error::Error as LapinError;

/// Details the various errors of the crate.
#[derive(Debug)]
pub enum Error {
    Lapin(LapinError),
    Io(IoError)
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
