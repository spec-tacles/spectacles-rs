use std::{
    io::{Error as IoError, ErrorKind},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use futures::{
    future::Future,
    stream::{SplitStream, Stream},
    sync::mpsc::{self, UnboundedSender},
    Sink,
};
use native_tls::TlsConnector;
use parking_lot::Mutex;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::timer::Interval;
use tokio_dns::TcpStream;
use tokio_tls::TlsStream;
use tokio_tungstenite::stream::Stream as TungsteniteStream;
use tokio_tungstenite::{
    tungstenite::{
        handshake::client::Request,
        protocol::{Message as WebsocketMessage, WebSocketConfig},
        Error as TungsteniteError,
    },
    WebSocketStream,
};
use url::Url;

use spectacles_model::gateway::{
    EventPayload, GatewayEvent, HeartbeatPacket, HelloPacket, IdentifyPacket, IdentifyProperties,
    Opcodes, ReadyPacket, ReceivePacket, ResumeSessionPacket, ResumedPacket,
};

use crate::{
    constants::{GATEWAY_URL, GATEWAY_VERSION},
    errors::{Error, Result},
};

pub type ShardSplitStream =
    SplitStream<WebSocketStream<TungsteniteStream<TokioTcpStream, TlsStream<TokioTcpStream>>>>;

/// A Spectacles Gateway shard.
#[derive(Clone)]
pub struct Shard {
    /// The bot token that this shard will use.
    pub token: String,
    /// The shard's info. Includes the shard's ID and the total amount of shards.
    pub info: [u64; 2],
    /// The session ID of this shard, if applicable.
    pub session_id: Option<String>,
    /// The interval at which a heartbeat is made.
    pub interval: Option<u64>,
    /// The channel which is used to send websocket messages.
    pub sender: Arc<Mutex<UnboundedSender<WebsocketMessage>>>,
    /// The shard's message stream, which is used to receive messages.
    pub stream: Arc<Mutex<Option<ShardSplitStream>>>,
    /// Used to determine whether or not the shard is currently in a state of connecting.
    current_state: Arc<Mutex<String>>,
    /// This shard's current heartbeat.
    pub heartbeat: Arc<Mutex<Heartbeat>>,
}

/// A shard's heartbeat information.
#[derive(Debug, Copy, Clone)]
pub struct Heartbeat {
    pub acknowledged: bool,
    pub seq: u64,
}

impl Heartbeat {
    fn new() -> Heartbeat {
        Self {
            acknowledged: false,
            seq: 0,
        }
    }
}

impl Shard {
    /// Creates a new Discord Shard, with the provided token.
    pub fn new(token: String, info: [u64; 2]) -> impl Future<Item = Shard, Error = Error> {
        Shard::begin_connection(GATEWAY_URL, info[0]).map(move |(sender, stream)| Shard {
            token,
            session_id: None,
            info,
            interval: None,
            sender: Arc::new(Mutex::new(sender)),
            current_state: Arc::new(Mutex::new(String::from("handshake"))),
            stream: Arc::new(Mutex::new(Some(stream))),
            heartbeat: Arc::new(Mutex::new(Heartbeat::new())),
        })
    }

    pub fn fulfill_gateway(
        &mut self,
        mess: WebsocketMessage,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let info = self.info.clone();
        let current_state = self.current_state.lock().clone();
        let packet: ReceivePacket = self.resolve_packet(&mess).unwrap();

        match packet.op {
            Opcodes::Dispatch => {
                if let Some(GatewayEvent::READY) = packet.t {
                    let ready: ReadyPacket = serde_eetf::from_bytes(packet.d.as_slice()).unwrap();
                    *self.current_state.lock() = "connected".to_string();
                    self.session_id = Some(ready.session_id.clone());
                    trace!(
                        "[Shard {}] Received ready, set session ID as {}",
                        &info[0],
                        ready.session_id
                    )
                };
            }
            Opcodes::Hello => {
                let hello: HelloPacket = serde_eetf::from_bytes(packet.d.as_slice()).unwrap();
                if hello.heartbeat_interval > 0 {
                    self.interval = Some(hello.heartbeat_interval);
                }
                if current_state == "handshake".to_string() {
                    let dur = Duration::from_millis(hello.heartbeat_interval);
                    tokio::spawn(Shard::begin_interval(self.clone(), dur));
                    trace!("[Shard {}] Identifying with the gateway.", &info[0]);
                    if let Err(e) = self.identify() {
                        warn!(
                            "[Shard {}] Failed to identify with gateway. {:?}",
                            &info[0], e
                        );
                    };
                }
                return Box::new(self.autoreconnect().map_err(|_| ()));
            }
            Opcodes::HeartbeatAck => {
                let mut hb = self.heartbeat.lock().clone();
                hb.acknowledged = true;
            }
            _ => {}
        };
        Box::new(futures::future::ok(()))
    }

    /// Identifies a shard with Discord.
    pub fn identify(&mut self) -> Result<()> {
        let sender = self.sender.clone();
        Shard::send_binary(
            &sender,
            IdentifyPacket {
                token: self.token.clone(),
                shard: self.info.clone(),
                version: GATEWAY_VERSION,
                large_threshold: 250,
                presence: None,
                compress: false,
                properties: IdentifyProperties {
                    os: std::env::consts::OS.to_string(),
                    browser: String::from("spectacles-rs"),
                    device: String::from("spectacles-rs"),
                },
            }
            .to_bytes()
            .unwrap(),
        )
    }

    /// Attempts to automatically reconnect the shard to Discord.
    pub fn autoreconnect(&mut self) -> Box<Future<Item = (), Error = Error> + Send> {
        if self.session_id.is_some() && self.heartbeat.lock().seq > 0 {
            Box::new(self.resume())
        } else {
            Box::new(self.reconnect())
        }
    }

    /// Makes a request to reconnect the shard.
    pub fn reconnect(&mut self) -> impl Future<Item = (), Error = Error> + Send {
        info!("[Shard {}] Perfoming reconnect to gateway.", &self.info[0]);
        *self.current_state.lock() = "reconnecting".to_string();
        self.reset_state()
            .expect(format!("[Shard {}] Failed to reset shard state.", self.info[0]).as_ref());
        self.dial_gateway()
    }

    fn reset_state(&mut self) -> Result<()> {
        *self.current_state.lock() = "disconnected".to_string();
        self.session_id = None;
        let mut hb = self.heartbeat.lock();
        hb.seq = 0;

        Ok(())
    }

    /// Resumes a shard's past session.
    pub fn resume(&mut self) -> impl Future<Item = (), Error = Error> + Send {
        let seq = self.heartbeat.lock().seq;
        let token = self.token.clone();
        let state = self.current_state.clone();
        let session = self.session_id.clone();
        let sender = self.sender.clone();

        self.dial_gateway().then(move |result| {
            if result.is_err() {
                return result;
            }
            *state.lock() = "resuming".to_string();
            Shard::send_binary(
                &sender,
                ResumeSessionPacket {
                    token,
                    seq,
                    session_id: session.unwrap(),
                }
                .to_bytes()
                .unwrap(),
            );

            Ok(())
        })
    }
    /// Resolves a Websocket message into a ReceivePacket struct.
    pub fn resolve_packet(&self, mess: &WebsocketMessage) -> Result<ReceivePacket> {
        let res = mess.clone();
        let res = res.into_data();
        let s: ReceivePacket = serde_eetf::from_bytes(res.as_slice()).unwrap();
        Ok(s)
    }

    /// Sends a payload to the Discord Gateway.
    pub fn send(&self, message: WebsocketMessage) -> Result<()> {
        self.sender
            .lock()
            .start_send(message)
            .map(|_| ())
            .map_err(From::from)
    }

    fn heartbeat(&mut self) -> Result<()> {
        trace!("[Shard {}] Sending heartbeat.", self.info[0]);
        let seq = self.heartbeat.lock().seq;
        let sender = self.sender.clone();
        Shard::send_binary(&sender, HeartbeatPacket { seq }.to_bytes().unwrap())
    }

    fn dial_gateway(&mut self) -> impl Future<Item = (), Error = Error> + Send {
        let info = self.info.clone();
        *self.current_state.lock() = String::from("connected");
        let state = self.current_state.clone();
        let orig_sender = self.sender.clone();
        let orig_stream = self.stream.clone();
        let heartbeat = self.heartbeat.clone();

        Shard::begin_connection(GATEWAY_URL, info[0]).map(move |(sender, stream)| {
            *orig_sender.lock() = sender;
            *heartbeat.lock() = Heartbeat::new();
            *state.lock() = String::from("handshake");
            *orig_stream.lock() = Some(stream);
        })
    }

    fn begin_interval(mut shard: Shard, duration: Duration) -> impl Future<Item = (), Error = ()> {
        let info = shard.info.clone();
        Interval::new(Instant::now(), duration)
            .map_err(move |err| {
                warn!(
                    "[Shard {}] Failed to begin heartbeat interval. {:?}",
                    info[0], err
                );
            })
            .for_each(move |_| {
                if let Err(r) = shard.heartbeat() {
                    warn!("[Shard {}] Failed to perform heartbeat. {:?}", info[0], r);
                    return Err(());
                }
                Ok(())
            })
    }

    fn send_binary(
        sender: &Arc<Mutex<UnboundedSender<WebsocketMessage>>>,
        value: Vec<u8>,
    ) -> Result<()> {
        sender
            .lock()
            .start_send(WebsocketMessage::Binary(value))
            .map(|_| ())
            .map_err(From::from)
    }

    fn begin_connection(
        ws: &str,
        shard_id: u64,
    ) -> impl Future<Item = (UnboundedSender<WebsocketMessage>, ShardSplitStream), Error = Error>
    {
        let url = Url::from_str(ws).expect("Invalid Websocket URL has been provided.");
        let req = Request::from(url);
        let (host, port) = Shard::get_addr_info(&req);
        let tlsconn = TlsConnector::new().unwrap();
        let tlsconn = tokio_tls::TlsConnector::from(tlsconn);

        let socket = TcpStream::connect((host.as_ref(), port));
        let handshake = socket.and_then(move |socket| {
            info!("[Shard {}] Beginning handshake with gateway.", shard_id);
            tlsconn
                .connect(host.as_ref(), socket)
                .map(|s| TungsteniteStream::Tls(s))
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });
        let stream = handshake.and_then(|mut stream| {
            tokio_tungstenite::stream::NoDelay::set_nodelay(&mut stream, true).map(move |()| stream)
        });
        let stream = stream.and_then(move |stream| {
            tokio_tungstenite::client_async_with_config(
                req,
                stream,
                Some(WebSocketConfig {
                    max_message_size: Some(usize::max_value()),
                    max_frame_size: Some(usize::max_value()),
                    ..Default::default()
                }),
            )
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });

        stream
            .map(move |(wstream, _)| {
                let (tx, rx) = mpsc::unbounded();
                let (sink, stream) = wstream.split();
                tokio::spawn(
                    rx.map_err(|err| {
                        error!("Failed to select sink. {:?}", err);
                        TungsteniteError::Io(IoError::new(
                            ErrorKind::Other,
                            "Error whilst attempting to select sink.",
                        ))
                    })
                    .forward(sink)
                    .map(|_| ())
                    .map_err(|_| ()),
                );

                (tx, stream)
            })
            .from_err()
    }

    fn get_addr_info(req: &Request) -> (String, u16) {
        let host = req
            .url
            .host_str()
            .expect("Could Not parse the Websocket Host.");
        let port = req
            .url
            .port_or_known_default()
            .expect("Could not parse the websocket port.");

        (host.to_string(), port)
    }
}
