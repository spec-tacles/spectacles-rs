use std::{
    borrow::BorrowMut,
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
use futures::sync::mpsc::UnboundedSender;
use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock};
use tokio::await;
use tokio::prelude::*;
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

/// A collection of shards, keyed by shard ID.
pub type ShardMap = HashMap<usize, Arc<Mutex<Shard>>>;
type MessageStream = UnboundedReceiver<(Arc<Mutex<Shard>>, TungsteniteMessage)>;

// A stream of shards being spawned and emitting the ready event.
pub struct Spawner {
    inner: UnboundedReceiver<Arc<Mutex<Shard>>>
}

impl Spawner {
    fn new(receiver: UnboundedReceiver<Arc<Mutex<Shard>>>) -> Self {
        Spawner { inner: receiver }
    }
}

impl Stream for Spawner {
    type Item = Arc<Mutex<Shard>>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

/// A stream of shard events.
pub struct EventHandle {
    inner: UnboundedReceiver<ReceivePacket>
}

impl EventHandle {
    fn new(receiver: UnboundedReceiver<ReceivePacket>) -> Self {
        EventHandle { inner: receiver }
    }
}

impl Stream for EventHandle {
    type Item = ReceivePacket;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}


pub struct ShardManager {
    /// The token used by this manager to spawn shards.
    pub token: String,
    /// The total amount of shards that this manager will attempt to spawn.
    pub total_shards: usize,
    /// A collection of shards that have been spawned.
    pub shards: Arc<RwLock<ShardMap>>,
    message_stream: Option<MessageStream>,
    event_sender: Option<UnboundedSender<ReceivePacket>>
}

impl ShardManager {
    pub async fn new(token: String, strategy: ShardStrategy) -> Result<ShardManager> {
        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };

        use reqwest::r#async::Client;
        let mut res = await!(Client::new().get(&format!("{}/gateway/bot", API_BASE))
            .header("Authorization", token.clone()).send())?;
        let gb: GatewayBot = await!(res.json())?;
        let total_shards = match strategy {
            ShardStrategy::Recommended => gb.shards,
            ShardStrategy::SpawnAmount(int) => int
        };

        Ok(ShardManager {
            token,
            total_shards,
            shards: Arc::new(RwLock::new(HashMap::new())),
            event_sender: None,
            message_stream: None
        })
    }

    pub fn begin_spawn(&mut self) -> (Spawner, EventHandle) {
        let (sender, receiver) = unbounded();
        self.message_stream = Some(receiver);
        let (tx, rx) = unbounded();
        let shard_count = self.total_shards.clone();
        let token = self.token.clone();
        let shardmap = self.shards.clone();
        debug!("Attempting to spawn {} shards.", &shard_count);

        tokio::spawn_async(async move {
            for id in 0..shard_count {
                await!(Delay::new(Instant::now() + Duration::from_secs(6)))
                    .expect("Failed to delay shard spawn.");
                let count = shard_count.clone();
                let shard = Arc::new(
                    Mutex::new(await!(Shard::new(token.clone(), [id, count])).expect("Failed to create shard"))
                );
                shardmap.write().insert(id, shard.clone());
                let sink = MessageSink {
                    shard: shard.clone(),
                    sender: sender.clone(),
                };
                let split = shard.lock().stream.lock().take().unwrap().map_err(MessageSinkError::from);
                tokio::spawn(split.forward(sink)
                    .map(|_| ())
                    .map_err(|e| error!("Failed to forward shard messages to the sink. {:?}", e))
                );
                tx.unbounded_send(shard).expect("Failed to send shard to stream");
            };
            info!("All shards have completed spawn.");
        });
        let event_handle = self.start_event_stream();

        (Spawner::new(rx), event_handle)
    }

    fn start_event_stream(&mut self) -> EventHandle {
        let mut stream = self.message_stream.take().unwrap();
        let (sender, receiver) = unbounded();
        self.event_sender = Some(sender.clone());
        tokio::spawn_async(async move {
            while let Some(Ok((mut shard, message))) = await!(stream.next()) {
                let current_shard = shard.borrow_mut();
                let mut shard = current_shard.lock().clone();
                trace!("Websocket message received: {:?}", &message.clone());
                let event = shard.resolve_packet(&message.clone()).expect("Failed to parse the shard message");
                if let Opcodes::Dispatch = event.op {
                    sender.unbounded_send(event.clone()).expect("Failed to send shard event to stream");
                };
                let action = shard.fulfill_gateway(event.clone());
                if let Ok(ShardAction::Autoreconnect) = action {
                    await!(shard.autoreconnect()).expect("Shard failed to autoreconnect.");
                } else if let Ok(ShardAction::Identify) = action {
                    debug!("[Shard {}] Identifying with the gateway.", &shard.info[0]);
                    if let Err(e) = shard.identify() {
                        warn!("[Shard {}] Failed to identify with gateway. {:?}", &shard.info[0], e);
                    };
                } else if let Ok(ShardAction::Reconnect) = action {
                    await!(shard.reconnect()).expect("Failed to reconnect shard.");
                    info!("[Shard {}] Reconnection successful.", &shard.info[0]);
                } else if let Ok(ShardAction::Resume) = action {
                    await!(shard.resume()).expect("Failed to resume shard session.");
                    info!("[Shard {}] Successfully resumed session.", &shard.info[0]);
                };
            }
        });

        EventHandle::new(receiver)
    }
}
