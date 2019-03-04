use std::{
    io::{Error as IoError, ErrorKind},
    str::FromStr,
    sync::Arc,
};

use futures::{
    future::Future,
    Sink,
    stream::{SplitStream, Stream},
    sync::mpsc::{self, UnboundedSender}
};
use native_tls::TlsConnector;
use parking_lot::Mutex;
use tokio::net::TcpStream as TokioTcpStream;
use tokio_dns::TcpStream;
use tokio_tls::TlsStream;
use tokio_tungstenite::{
    tungstenite::{
        Error as TungsteniteError,
        handshake::client::Request,
        protocol::{Message as WebsocketMessage, WebSocketConfig},
    },
    WebSocketStream
};
use tokio_tungstenite::stream::Stream as TungsteniteStream;
use url::Url;

use spectacles_model::gateway::Opcodes;

use crate::{
    constants::{GATEWAY_URL, GATEWAY_VERSION},
    errors::{Error, Result},
};

pub type ShardSplitStream = SplitStream<WebSocketStream<TungsteniteStream<TokioTcpStream, TlsStream<TokioTcpStream>>>>;

/// A shard's heartbeat information.
#[derive(Debug, Copy, Clone)]
pub struct Heartbeat {
    pub sequence: u64
}
/// A Spectacles Gateway shard.
#[derive(Clone)]
pub struct Shard {
    /// The bot token that this shard will use.
    pub token: String,
    /// The shard's info. Includes the shard's ID and the total amount of shards.
    pub info: [u64; 2],
    /// The session ID of this shard, if applicable.
    pub session_id: Option<String>,
    /// The channel which is used to send websocket messages.
    pub sender: Arc<Mutex<UnboundedSender<WebsocketMessage>>>,
    /// The shard's message stream, which is used to receive messages.
    pub stream: Arc<Mutex<ShardSplitStream>>,
    has_acked: bool,
}

impl Shard {
    /// Creates a new Discord Shard, with the provided token.
    pub fn new(token: String, info: [u64; 2]) -> impl Future<Item = Shard, Error = Error> {
        Shard::begin_connection(GATEWAY_URL, info[0])
            .map(move |(sender, stream)| {
                Shard {
                    token,
                    session_id: None,
                    info,
                    sender: Arc::new(Mutex::new(sender)),
                    stream: Arc::new(Mutex::new(stream)),
                    has_acked: true,
                }
            })
    }

    /// Identifies a shard with Discord.
    pub fn identify(&mut self) -> Result<()> {
        self.send_json(&json!({
            "op": Opcodes::Identify as i32,
            "d": {
                "large_threshold": 250,
                "token": self.token,
                "shard": self.info,
                "v": GATEWAY_VERSION,
                "properties": {
                    "$os": std::env::consts::OS,
                    "$browser": "spectacles-rs",
                    "$device": "spectacles"
                }
            }
        }))
    }

    /// Sends a payload to the Discord Gateway.
    pub fn send(&self, message: WebsocketMessage) -> Result<()> {
        self.sender.lock().start_send(message)
            .map(|_| ())
            .map_err(From::from)
    }


    fn send_json(&mut self, value: &serde_json::Value) -> Result<()> {
        let json = serde_json::to_string(value)?;
        self.send(WebsocketMessage::text(json))
    }

    fn heartbeat(&self) {

    }

    fn begin_connection(ws: &str, shard_id: u64) -> impl Future<Item = (UnboundedSender<WebsocketMessage>, ShardSplitStream), Error = Error> {
        let url = Url::from_str(ws).expect("Invalid Websocket URL has been provided.");
        let req = Request::from(url);
        let (host, port) = Shard::get_addr_info(&req);
        let tlsconn = TlsConnector::new().unwrap();
        let tlsconn = tokio_tls::TlsConnector::from(tlsconn);

        let socket = TcpStream::connect((host.as_ref(), port));
        let handshake = socket.and_then(move |socket| {
            info!("[Shard {}] Beginning handshake with gateway.", shard_id);
            tlsconn.connect(host.as_ref(), socket)
                .map(|s| TungsteniteStream::Tls(s))
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });
        let stream = handshake.and_then(|mut stream| {
            tokio_tungstenite::stream::NoDelay::set_nodelay(&mut stream, true)
                .map(move |()| stream)
        });
        let stream = stream.and_then(move |stream| {
            tokio_tungstenite::client_async_with_config(req, stream, Some(WebSocketConfig {
                max_message_size: Some(usize::max_value()),
                max_frame_size: Some(usize::max_value()),
                ..Default::default()
            })).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });

        stream.map(move |(wstream, _)| {
            let (tx, rx) = mpsc::unbounded();
            let (sink, stream) = wstream.split();
            tokio::spawn(rx.map_err(|err| {
                error!("Failed to select sink. {:?}", err);
                TungsteniteError::Io(IoError::new(ErrorKind::Other, "Error whilst attempting to select sink."))
            }).forward(sink).map(|_| ()).map_err(|_| ()));

            (tx, stream)
        }).from_err()
    }

    fn get_addr_info(req: &Request) -> (String, u16) {
        let host = req.url.host_str().expect("Could Not parse the Websocket Host.");
        let port = req.url.port_or_known_default().expect("Could not parse the websocket port.");

        (host.to_string(), port)
    }
}