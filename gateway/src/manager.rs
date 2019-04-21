use std::{
    sync::Arc,
    time::Duration,
    time::Instant
};

use futures::{
    future::Future,
    Poll,
    Stream,
    sync::mpsc::{unbounded, UnboundedReceiver}
};
use futures::future::Loop;
use futures::sync::mpsc::UnboundedSender;
use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock};
use tokio::timer::Delay;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

use spectacles_model::gateway::{GatewayBot, Opcodes, ReceivePacket};

use crate::{
    constants::API_BASE,
    errors::*,
    queue::{MessageSink, MessageSinkError},
    shard::{Shard, ShardAction}
};

/// The strategy in which you would like to spawn shards.
#[derive(Clone)]
pub enum ShardStrategy {
    /// The spawner will automatically spawn shards based on the amount recommended by Discord.
    Recommended,
    /// Spawns shards according to the amount specified, starting from shard 0.
    SpawnAmount(usize)
}

#[derive(Clone)]
/// Information about a Discord Gateway event received for a shard.
pub struct ShardEvent {
    /// The shard which emitted this event.
    pub shard: ManagerShard,
    /// The Discord Gateway packet that the event contains.
    pub packet: ReceivePacket,
}

/// A collection of shards, keyed by shard ID.
pub type ShardMap = HashMap<usize, Arc<Mutex<Shard>>>;
/// An alias for a shard spawned with the sharding manager.
pub type ManagerShard = Arc<Mutex<Shard>>;
type MessageStream = UnboundedReceiver<(ManagerShard, TungsteniteMessage)>;

/// A stream of shards being spawned and emitting the ready event.
pub struct Spawner {
    inner: UnboundedReceiver<ManagerShard>
}

impl Spawner {
    fn new(receiver: UnboundedReceiver<ManagerShard>) -> Self {
        Spawner { inner: receiver }
    }
}

impl Stream for Spawner {
    type Item = ManagerShard;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

/// A stream of incoming Discord events for a shard.
pub struct EventHandler {
    inner: UnboundedReceiver<ShardEvent>
}

impl EventHandler {
    fn new(receiver: UnboundedReceiver<ShardEvent>) -> Self {
        EventHandler { inner: receiver }
    }
}

impl Stream for EventHandler {
    type Item = ShardEvent;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

#[derive(Clone)]
struct SpawnerLoop {
    shardmap: Arc<RwLock<ShardMap>>,
    sink_tx: UnboundedSender<(ManagerShard, TungsteniteMessage)>,
    current: usize,
    total: usize,
    sender: UnboundedSender<ManagerShard>,
    token: String,
    ws: String,
}

/// The central hub for all shards, where shards are spawned and maintained.
pub struct ShardManager {
    /// The token used by this manager to spawn shards.
    pub token: String,
    /// The total amount of shards that this manager will attempt to spawn.
    pub total_shards: usize,
    /// A collection of shards that have been spawned.
    pub shards: Arc<RwLock<ShardMap>>,
    event_sender: Option<UnboundedSender<ShardEvent>>,
    message_stream: Option<MessageStream>,
    ws_uri: String
}

impl ShardManager {
    /// Creates a new cluster, with the provided Discord API token.
    pub fn new(token: String, strategy: ShardStrategy) -> impl Future<Item=ShardManager, Error=Error> {
        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };

        use reqwest::r#async::Client;
        Client::new().get(format!("{}/gateway/bot", API_BASE).as_str())
            .header("Authorization", token.clone()).send()
            .and_then(|mut resp| resp.json::<GatewayBot>())
            .map_err(Error::from)
            .map(move |gb| {
                let shard_count = match strategy {
                    ShardStrategy::Recommended => gb.shards,
                    ShardStrategy::SpawnAmount(int) => int
                };

                Self {
                    token,
                    total_shards: shard_count,
                    shards: Arc::new(RwLock::new(HashMap::new())),
                    event_sender: None,
                    message_stream: None,
                    ws_uri: gb.url
                }
            })
    }

    /// Spawns shards up to the specified amount and identifies them with Discord.
    pub fn start_spawn(&mut self) -> (Spawner, EventHandler) {
        let (sender, receiver) = unbounded();
        self.message_stream = Some(receiver);
        let (tx, rx) = unbounded();
        debug!("Attempting to spawn {} shards.", &self.total_shards);
        let initial = SpawnerLoop {
            current: 0,
            shardmap: Arc::clone(&self.shards),
            sink_tx: sender.clone(),
            sender: tx,
            total: self.total_shards,
            token: self.token.clone(),
            ws: self.ws_uri.clone(),
        };

        tokio::spawn(futures::future::loop_fn(initial, move |state| {
            Delay::new(Instant::now() + Duration::from_secs(6)).from_err()
                .map(|_| state)
                .and_then(move |mut state| {
                    Shard::new(state.token.clone(), [state.current, state.total], state.ws.clone())
                        .map(move |shard| {
                            let wrapped = ManagerShard::new(Mutex::new(shard));
                            state.shardmap.write().insert(wrapped.lock().info[0], Arc::clone(&wrapped));
                            let sink = MessageSink {
                                shard: Arc::clone(&wrapped),
                                sender: state.sink_tx.clone(),
                            };
                            let split = wrapped.lock().stream.lock().take().unwrap().map_err(MessageSinkError::from);
                            tokio::spawn(split.forward(sink)
                                .map(|_| ())
                                .map_err(|e| error!("Failed to forward shard messages to the sink. {:?}", e))
                            );
                            state.sender.unbounded_send(wrapped).expect("Failed to send shard to stream");
                            state.current += 1;

                            state
                        })
                })
                .map(|state| {
                    if state.current == state.total {
                        Loop::Break(())
                    } else {
                        Loop::Continue(state)
                    }
                })
        }).map_err(|err| {
            error!("Failed in sharding process. {:?}", err);
        }));

        (Spawner::new(rx), self.start_event_stream())
    }

    fn start_event_stream(&mut self) -> EventHandler {
        let stream = self.message_stream.take().unwrap();
        let (sender, receiver) = unbounded();
        self.event_sender = Some(sender.clone());

        tokio::spawn(stream.for_each(move |(shard, message)| {
            trace!("Websocket message received: {:?}", &message);
            let event = shard.lock().resolve_packet(&message).expect("Failed to parse the shard message");
            if let Opcodes::Dispatch = event.op {
                sender.unbounded_send(ShardEvent {
                    packet: event.clone(),
                    shard: Arc::clone(&shard),
                }).expect("Failed to send shard event to stream");
            };
            let action = shard.lock().fulfill_gateway(event.clone()).expect("Failed to fufill gateway message");

            match action {
                ShardAction::Autoreconnect => {
                    let sd = Arc::clone(&shard);
                    tokio::spawn(shard.lock().autoreconnect().map(move |_| {
                        info!("[Shard {}] Auto reconnection successful.", sd.lock().info[0]);
                    }).map_err(|err| {
                        error!("Failed to auto reconnect shard. {}", err);
                    }));
                },
                ShardAction::Identify => {
                    let info = shard.lock().info;
                    debug!("[Shard {}] Identifying with the gateway.", &info[0]);
                    if let Err(e) = shard.lock().identify() {
                        warn!("[Shard {}] Failed to identify with gateway. {:?}", &info[0], e);
                    };
                },
                ShardAction::Reconnect => {
                    let sd = Arc::clone(&shard);
                    tokio::spawn(shard.lock().reconnect().map(move |_| {
                        info!("[Shard {}] Reconnection successful.", sd.lock().info[0]);
                    }).map_err(|err| {
                        error!("Shard failed to reconnect to the gateway. {}", err);
                    }));
                },
                ShardAction::Resume => {
                    let sd = Arc::clone(&shard);
                    tokio::spawn(shard.lock().resume().map(move |_| {
                        info!("[Shard {}] Successfully resumed session.", sd.lock().info[0]);
                    }).map_err(|err| {
                        error!("Shard failed to resume session. {}", err);
                    }));
                },
                _ => {}
            };

            Ok(())
        }));

        EventHandler::new(receiver)
    }
}