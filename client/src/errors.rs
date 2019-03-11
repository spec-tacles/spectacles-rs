use std::{
    error::Error as StdError,
    // result::Result as StdResult,
    fmt::{Display, Formatter, Result as FmtResult}
};

use spectacles_brokers::Error as BrokerError;
use spectacles_gateway::Error as GatewayError;

// pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Broker(BrokerError),
    Gateway(GatewayError)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Broker(e) => e.description(),
            Error::Gateway(e) => e.description()
        }
    }
}

impl From<BrokerError> for Error {
    fn from(err: BrokerError) -> Self {
        Error::Broker(err)
    }
}

impl From<GatewayError> for Error {
    fn from(err: GatewayError) -> Self {
        Error::Gateway(err)
    }
}