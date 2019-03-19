use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

use failure::{Compat, Fail};
use lapin_futures_native_tls::{
    error::Error as LapinTlsError,
    lapin::error::Error as LapinError,
};

/// Details the various errors of the crate.
#[derive(Debug)]
pub enum Error {
    Lapin(Compat<LapinError>),
    LapinTls(Compat<LapinTlsError>),
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
            Error::Lapin(e) => e.description(),
            Error::LapinTls(e) => e.description(),
            Error::Io(e) => e.description()
        }
    }
}

impl From<LapinError> for Error {
    fn from(err: LapinError) -> Self {
        Error::Lapin(err.compat())
    }
}

impl From<LapinTlsError> for Error {
    fn from(err: LapinTlsError) -> Self {
        Error::LapinTls(err.compat())
    }
}


impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}
