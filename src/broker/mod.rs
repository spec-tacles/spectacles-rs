use ::errors::{Result, Error};
use lapin_futures as lapin;
use tokio::net::TcpStream;
use futures::future::Future;
use std::net::SocketAddr;

pub struct MessageBroker {
    group: String,
    subgroup: String

}

impl MessageBroker {
    fn new(group: String, subgroup: String) -> MessageBroker {
        Self {
            group,
            subgroup
        }
    }

    fn connect(&self, addr: &SocketAddr) {

    }
}