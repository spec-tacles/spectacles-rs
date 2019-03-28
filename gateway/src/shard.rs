use std::{
    io::{Error as IoError, ErrorKind},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant}
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
use tokio::prelude::*;
use tokio::timer::Interval;
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

use spectacles_model::{
    gateway::{
        GatewayEvent,
        HeartbeatPacket,
        HelloPacket,
        IdentifyPacket,
        IdentifyProperties,
        Opcodes,
        ReadyPacket,
        ReceivePacket,
        ResumeSessionPacket,
        SendablePacket,
    },
    presence::{ClientActivity, ClientPresence, Status}
};

use crate::{
    constants::{GATEWAY_URL, GATEWAY_VERSION},
    errors::{Error, Result}
};

pub type ShardSplitStream = SplitStream<WebSocketStream<TungsteniteStream<TokioTcpStream, TlsStream<TokioTcpStream>>>>;

/// A Spectacles Gateway shard.
#[derive(Clone)]
pub struct Shard {
    /// The bot token that this shard will use.
    pub token: String,
    /// The shard's info. Includes the shard's ID and the total amount of shards.
    pub info: [usize; 2],
    /// The currently active presence for this shard.
    pub presence: ClientPresence,
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

/// Various actions that a shard can perform.
pub enum ShardAction {
    NoneAction,
    Autoreconnect,
    Reconnect,
    Identify,
    Resume
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
            seq: 0
        }
    }
}

impl Shard {
    /// Creates a new Discord Shard, with the provided token.
    pub async fn new(token: String, info: [usize; 2]) -> Result<Shard> {
        let (sender, stream) = await!(Shard::begin_connection(GATEWAY_URL))?;
        Ok(Shard {
            token,
            session_id: None,
            presence: ClientPresence {
                status: String::from("online"),
                ..Default::default()
            },
            info,
            interval: None,
            sender: Arc::new(Mutex::new(sender)),
            current_state: Arc::new(Mutex::new(String::from("handshake"))),
            stream: Arc::new(Mutex::new(Some(stream))),
            heartbeat: Arc::new(Mutex::new(Heartbeat::new())),
        })

    }

    pub fn fulfill_gateway(&mut self, packet: ReceivePacket) -> Result<ShardAction> {
        let info = self.info.clone();
        let current_state = self.current_state.lock().clone();
        match packet.op {
            Opcodes::Dispatch => {
                if let Some(GatewayEvent::READY) = packet.t {
                    let ready: ReadyPacket = serde_json::from_str(packet.d.get()).unwrap();
                    *self.current_state.lock() = "connected".to_string();
                    self.session_id = Some(ready.session_id.clone());
                    trace!("[Shard {}] Received ready, set session ID as {}", &info[0], ready.session_id)
                };
                Ok(ShardAction::NoneAction)
            }
            Opcodes::Hello => {
                if self.current_state.lock().clone() == "resume".to_string() {
                    return Ok(ShardAction::NoneAction)
                };
                let hello: HelloPacket = serde_json::from_str(packet.d.get()).unwrap();
                if hello.heartbeat_interval > 0 {
                    self.interval = Some(hello.heartbeat_interval);
                }
                if current_state == "handshake".to_string() {
                    let dur = Duration::from_millis(hello.heartbeat_interval);
                    tokio::spawn_async(Shard::begin_interval(self.clone(), dur));
                    return Ok(ShardAction::Identify);
                }
                Ok(ShardAction::Autoreconnect)
            },
            Opcodes::HeartbeatAck => {
                let mut hb = self.heartbeat.lock().clone();
                hb.acknowledged = true;
                Ok(ShardAction::NoneAction)
            },
            Opcodes::Reconnect => Ok(ShardAction::Reconnect),
            Opcodes::InvalidSession => {
                let invalid: bool = serde_json::from_str(packet.d.get()).unwrap();
                if !invalid {
                    Ok(ShardAction::Identify)
                } else { Ok(ShardAction::Resume) }
            },
            _ => Ok(ShardAction::NoneAction)
        }
    }

    /// Identifies a shard with Discord.
    pub fn identify(&mut self) -> Result<()> {
        let token = self.token.clone();
        let shard = self.info.clone();
        let presence = self.presence.clone();
        self.send_payload(IdentifyPacket {
            large_threshold: 250,
            token,
            shard,
            compress: false,
            presence: Some(presence),
            version: GATEWAY_VERSION,
            properties: IdentifyProperties {
                os: std::env::consts::OS.to_string(),
                browser: String::from("spectacles-rs"),
                device: String::from("spectacles-rs")
            }
        })
    }

    /// Attempts to automatically reconnect the shard to Discord.
    pub async fn autoreconnect(&mut self) -> Result<()> {
        if self.session_id.is_some() && self.heartbeat.lock().seq > 0 {
            await!(self.resume())?;

            Ok(())
        } else {
            await!(self.reconnect())?;

            Ok(())
        }
    }

    /// Makes a request to reconnect the shard.
    pub async fn reconnect(&mut self) -> Result<()> {
        debug!("[Shard {}] Attempting to reconnect to gateway.", &self.info[0]);
        self.reset_values().expect("[Shard] Failed to reset this shard for autoreconnecting.");
        await!(self.dial_gateway())?;

        Ok(())
    }

    /// Resumes a shard's past session.
    pub async fn resume(&mut self) -> Result<()> {
        debug!("[Shard {}] Attempting to resume gateway connection.", &self.info[0]);
        let seq = self.heartbeat.lock().seq;
        let token = self.token.clone();
        let session = self.session_id.clone();
        let sender = self.sender.clone();
        match await!(self.dial_gateway()) {
            Ok(_) => {
                let payload = ResumeSessionPacket {
                    session_id: session.unwrap(),
                    seq,
                    token,
                };
                send(&sender, WebsocketMessage::text(payload.to_json()?))
            },
            Err(e) => Err(e)
        }
    }
    /// Resolves a Websocket message into a ReceivePacket struct.
    pub fn resolve_packet(&self, mess: &WebsocketMessage) -> Result<ReceivePacket> {
        match mess {
            WebsocketMessage::Binary(v) => serde_json::from_slice(v),
            WebsocketMessage::Text(v) => serde_json::from_str(v),
            _ => unreachable!("Invalid type detected."),
        }.map_err(Error::from)
    }

    /// Sends a payload to the Discord Gateway.
    pub fn send_payload<T: SendablePacket>(&self, payload: T) -> Result<()> {
        let json = payload.to_json()?;
        send(&self.sender, WebsocketMessage::text(json))
    }


    /// Change the status of the current shard.
    pub fn change_status(&mut self, status: Status) -> Result<()> {
        self.presence.status = status.to_string();
        let oldpresence = self.presence.clone();
        self.change_presence(oldpresence)
    }

    /// Change the activity of the current shard.
    pub fn change_activity(&mut self, activity: ClientActivity) -> Result<()> {
        self.presence.game = Some(activity);
        let oldpresence = self.presence.clone();
        self.change_presence(oldpresence)
    }

    /// Change the presence of the current shard.
    pub fn change_presence(&mut self, presence: ClientPresence) -> Result<()> {
        debug!("[Shard {}] Sending a presence change payload. {:?}", self.info[0], presence.clone());
        self.send_payload(presence.clone())?;
        self.presence = presence;
        Ok(())
    }

    fn reset_values(&mut self) -> Result<()> {
        self.session_id = None;
        *self.current_state.lock() = "disconnected".to_string();

        let mut hb = self.heartbeat.lock();
        hb.acknowledged = true;
        hb.seq = 0;

        Ok(())
    }

    fn heartbeat(&mut self) -> Result<()> {
        debug!("[Shard {}] Sending heartbeat.", self.info[0]);
        let seq = self.heartbeat.lock().seq;

        self.send_payload(HeartbeatPacket { seq })
    }

    async fn dial_gateway(&mut self) -> Result<()> {
        *self.current_state.lock() = String::from("connected");
        let state = self.current_state.clone();
        let orig_sender = self.sender.clone();
        let orig_stream = self.stream.clone();
        let heartbeat = self.heartbeat.clone();

        let (sender, stream) = await!(Shard::begin_connection(GATEWAY_URL))?;
        *orig_sender.lock() = sender;
        *heartbeat.lock() = Heartbeat::new();
        *state.lock() = String::from("handshake");
        *orig_stream.lock() = Some(stream);

        Ok(())
    }


    async fn begin_interval(mut shard: Shard, duration: Duration) {
        let info = shard.info.clone();
        let mut stream = Interval::new(Instant::now(), duration)
            .map_err(move |err| {
                warn!("[Shard {}] Failed to begin heartbeat interval. {:?}", info[0], err);
            });
        if let Some(Ok(_)) = await!(stream.next()) {
            if let Err(r) = shard.heartbeat() {
                warn!("[Shard {}] Failed to perform heartbeat. {:?}", info[0], r);
            }
        };

    }

    async fn begin_connection(ws: &str) -> Result<(UnboundedSender<WebsocketMessage>, ShardSplitStream)> {
        let url = Url::from_str(ws).expect("Invalid Websocket URL has been provided.");
        let req = Request::from(url);
        let (host, port) = Shard::get_addr_info(&req);
        let tlsconn = TlsConnector::new().unwrap();
        let tlsconn = tokio_tls::TlsConnector::from(tlsconn);

        let socket = await!(TcpStream::connect((host.as_ref(), port)))?;
        let mut stream = await!(tlsconn.connect(host.as_ref(), socket)
            .map(|s| TungsteniteStream::Tls(s))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        )?;
        let stream = tokio_tungstenite::stream::NoDelay::set_nodelay(&mut stream, true)
            .map(move |()| stream)?;

        let (wstream, _) = await!(
            tokio_tungstenite::client_async_with_config(req, stream, Some(WebSocketConfig {
                max_message_size: Some(usize::max_value()),
                max_frame_size: Some(usize::max_value()),
                ..Default::default()
            })).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        )?;
        let (tx, rx) = mpsc::unbounded();
        let (sink, stream) = wstream.split();
        tokio::spawn(rx.map_err(|err| {
            error!("Failed to select sink. {:?}", err);
            TungsteniteError::Io(IoError::new(ErrorKind::Other, "Error whilst attempting to select sink."))
        }).forward(sink).map(|_| ()).map_err(|_| ()));

        Ok((tx, stream))
    }

    fn get_addr_info(req: &Request) -> (String, u16) {
        let host = req.url.host_str().expect("Could Not parse the Websocket Host.");
        let port = req.url.port_or_known_default().expect("Could not parse the websocket port.");

        (host.to_string(), port)
    }
}

fn send(sender: &Arc<Mutex<UnboundedSender<WebsocketMessage>>>, mess: WebsocketMessage) -> Result<()> {
    sender.lock().start_send(mess)
        .map(|_| ())
        .map_err(From::from)
}