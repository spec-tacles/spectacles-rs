use std::{
    str::FromStr,
    sync::Arc
};

use futures::{
    future::Future,
    stream::Stream,
    sync::mpsc::{self, UnboundedSender}
};
use parking_lot::Mutex;
use serde_json::Value;
use tokio_dns::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        handshake::client::Request,
        protocol::{Message, WebSocketConfig},
    }
};
use url::Url;

use crate::{
    cluster::Cluster,
    constants::GATEWAY_URL,
};

/// A Spectacles Gateway shard.
pub struct Shard {
    /// The unique ID of this shard.
    pub id: i32,
    /// The bot token that this shard will use.
    pub token: String,
    /// The session ID of this shard, if applicable.
    pub session_id: Option<String>,
    /// The cluster that this shard belongs to, if applicable.
    pub cluster: Option<Cluster>,
    /// Whether or not this shard has acknowedged.
    pub has_acked: bool,
}

impl Shard {
    /// Creates a new Discord Shard, with the provided token.
    pub fn new(token: String) {
        Shard::begin_connection(GATEWAY_URL)

    }

    /// Identifies a shard with Discord.
    pub fn identify(&self) {
        debug!("SHARD {} is identifying.", self.id);

    }

    /// Sends a payload to the Discord Gateway.
    pub fn send(&self, opcode: i32) {

    }

    fn begin_connection(ws: &str) {
        let url = Url::from_str(ws).expect("Invalid Websocket URL has been provided.");
        let host = url.host_str().expect("Could Not parse the Websocket Host.");
        let port = url.port().expect("Could not parse the websocket port.");

        TcpStream::connect((host, port))
            .from_err()
            .and_then(|stream| {
                tokio_tungstenite::client_async_with_config(url, stream, Some(WebSocketConfig {
                    max_message_size: Some(usize::max_value()),
                    max_frame_size: Some(usize::max_value()),
                    ..Default::default()
                }))
            })
            .and_then(|(wstream, _)| {
                debug!("Handshake to Gateway successful.");
                let (tx, rx) = mpsc::unbounded();
                let (sink, stream) = wstream.split();
                tokio::spawn(rx.forward(sink).map_err(|_| ()));

                (tx, stream)
            }).from_err()
    }
}