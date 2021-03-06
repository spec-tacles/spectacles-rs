use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    net::AddrParseError,
    result::Result as StdResult
};

use hyper::error::Error as HyperError;
use serde_json::Error as JsonError;
use tokio::timer::Error as TimerError;
use toml::de::Error as TomlDeError;

use spectacles_brokers::Error as BrokerError;
use spectacles_gateway::Error as GatewayError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Broker(BrokerError),
    Hyper(HyperError),
    Gateway(GatewayError),
    Addr(AddrParseError),
    Io(IoError),
    TomlDe(TomlDeError),
    Timer(TimerError),
    Json(JsonError),
    InvalidFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Addr(e) => e.description(),
            Error::Broker(e) => e.description(),
            Error::Io(e) => e.description(),
            Error::Timer(e) => e.description(),
            Error::Hyper(e) => e.description(),
            Error::Gateway(e) => e.description(),
            Error::Json(e) => e.description(),
            Error::TomlDe(e) => e.description(),
            Error::InvalidFile => "Invalid config file provided. Supported config files are JSON and TOML."
        }
    }
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Self {
        Error::Addr(err)
    }
}

impl From<TimerError> for Error {
    fn from(err: TimerError) -> Self {
        Error::Timer(err)
    }
}

impl From<BrokerError> for Error {
    fn from(err: BrokerError) -> Self {
        Error::Broker(err)
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Self {
        Error::Hyper(err)
    }
}

impl From<GatewayError> for Error {
    fn from(err: GatewayError) -> Self {
        Error::Gateway(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl From<TomlDeError> for Error {
    fn from(err: TomlDeError) -> Self {
        Error::TomlDe(err)
    }
}