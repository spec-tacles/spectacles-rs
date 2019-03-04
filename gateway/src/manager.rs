use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::Arc,
};
use std::time::Duration;
use std::time::Instant;

use futures::{
    future::Future,
    Stream,
    sync::mpsc::{
        channel, Receiver as MpscReceiver, Sender as MpscSender, unbounded,
        UnboundedReceiver, UnboundedSender
    }
};
use parking_lot::{Mutex, RwLock};
use tokio::timer::Delay;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

use spectacles_model::gateway::GatewayBot;

use crate::{
    constants::API_BASE,
    errors::Error,
    queue::{MessageSink, MessageSinkError, ReconnectQueue, ShardQueue},
    shard::Shard
};
use crate::EventHandler;
use crate::ManagerOptions;

/// The strategy in which you would like to spawn shards.
pub enum ShardStrategy {
    /// The spawner will automatically spawn shards based on the amount recommended by Discord.
    Recommended,
    /// Spawns shards according to the amount specified, starting from shard 0.
    SpawnAmount(u64)
}
pub type ShardMap = HashMap<u64, Arc<Mutex<Shard>>>;
/// An organized group of Discord gateway shards.
pub struct ShardManager<H: EventHandler + Send + Sync + 'static> {
    /// The token that this shard uses to connect to the Discord API.
    pub token: String,
    /// The options used to configure this shard manager.
    pub options: ManagerOptions<H>,
    /// A collection of shards that have been spawned.
    pub shards: Arc<RwLock<ShardMap>>,
    queue_sender: MpscSender<u64>,
    queue_receiver: Option<MpscReceiver<u64>>,
    reconnect_queue: ShardQueue,
    message_stream: Option<UnboundedReceiver<(Arc<Mutex<Shard>>, TungsteniteMessage)>>,
}

impl <H: EventHandler + Send + Sync + 'static> ShardManager<H> {
    /// Creates a new cluster, with the provided Discord API token.
    pub fn new(token: String, options: ManagerOptions<H>) -> Self {
        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };

        let (queue_sender, queue_receiver) = channel(0);

        Self {
            token,
            reconnect_queue: ShardQueue::new(1),
            queue_sender,
            options,
            queue_receiver: Some(queue_receiver),
            message_stream: None,
            shards: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Spawns shards up to the specified amount and identifies them with Discord.
    pub fn spawn(&mut self, shardcount: u64) -> Box<Future<Item = (), Error = Error> + Send> {
        if self.reconnect_queue.queue.capacity() < shardcount as usize {
            self.reconnect_queue.queue.reserve(shardcount as usize);
        }

        info!("[Manager] Attempting to spawn {} shards.", &shardcount);
        for i in 0..shardcount {
            trace!("[Manager] Sending shard {} to queue.", &i);
            tokio::spawn(self.reconnect_queue.push_back(i)
                .map_err(move |_| error!("[Manager] Failed to place Shard {} into reconnect queue.", i))
            );
        }

        let (sender, receiver) = unbounded();
        self.message_stream = Some(receiver);
        let mut queue_sender = self.queue_sender.clone();
        let receiver = self.queue_receiver.take().unwrap();
        let token = self.token.clone();
        let newsender = sender.clone();
        let shards = self.shards.clone();

        tokio::spawn(self.reconnect_queue.pop_front()
            .and_then(move |shard| {
                let shard = shard.expect("Shard queue is empty.");
                queue_sender.try_send(shard).expect("Failed to send starting shard.");
                futures::future::ok(())
            })
            .map_err(|_| error!("Failed to pop shard in reconnect queue."))
            .and_then(move |_| Self::receive_chan(
                receiver,
                token,
                shardcount,
                newsender,
                shards
            ))
        );
        Box::new(futures::future::ok(()))
    }

    pub fn events(self) -> impl Future<Item = (), Error = ()> + 'static {
        let messages = self.message_stream.unwrap();
        let handler = self.options.handler;
        let future = messages.for_each(move |(mut shard, message)| {
            let shard = shard.borrow_mut().lock();
            let event = shard.resolve_packet(&message).expect("Failed to parse the shard message.");
            shard.fufill_gateway(&event);
            handler.on_packet(event)
        });
        future
    }

    fn receive_chan(
        receiver: MpscReceiver<u64>,
        token: String,
        shardcount: u64,
        sender: UnboundedSender<(Arc<Mutex<Shard>>, TungsteniteMessage)>,
        shardmap: Arc<RwLock<ShardMap>>
    ) -> impl Future<Item = (), Error = ()> {
        receiver.for_each(move |shard_id| {
            trace!("[Manager] Received notification to shart Shard {}.", shard_id);
            let shardmap = shardmap.clone();
            let token = token.clone();
            let sender = sender.clone();
            Delay::new(Instant::now() + Duration::from_secs(5))
                .map_err(|err| error!("Failed to pause before spawning the next shard. {:?}", err))
                .and_then(move |_| {
                    tokio::spawn(Self::create_shard(token, [shard_id, shardcount], sender)
                        .map(move |shard| {
                            shardmap.write().insert(shard_id, shard);
                        })
                    );
                    futures::future::ok(())
                })
        }).map_err(|_| ())
    }
    /// Spawn the recommended amount of shards according to Discord for this token.
    fn get_recommended(self) -> impl Future<Item = u64, Error = Error> {
        self.get_gateway().map(|gb| gb.shards)
    }

    fn create_shard(
        token: String,
        info: [u64; 2],
        sender: UnboundedSender<(Arc<Mutex<Shard>>, TungsteniteMessage)>
    ) -> impl Future<Item = Arc<Mutex<Shard>>, Error = ()> {
        Shard::new(token.clone(), info)
            .and_then(|res| {
                let shard = Arc::new(Mutex::new(res));
                let sink = MessageSink {
                    shard: shard.clone(),
                    sender,
                };
                tokio::spawn(Box::new(shard.lock()
                    .stream.lock().take()
                    .unwrap()
                    .map_err(MessageSinkError::from)
                    .forward(sink)
                    .map(|_| ())
                    .map_err(|e| error!("[Manager] Failed to forward shard messages to the sink. {:?}", e)))
                );
                futures::future::ok(shard)
            })
            .map_err(move |err| error!("[Manager] Failed to start Shard {}. {:?}", info[0], err))

    }

    fn get_gateway(&self) -> impl Future<Item = GatewayBot, Error = Error> {
        use reqwest::r#async::Client;
        Client::new().get(format!("{}/gateway/bot", API_BASE).as_str())
            .header("Authorization", self.token.clone())
            .send()
            .and_then(|mut resp| resp.json::<GatewayBot>())
            .map(|gb| gb)
            .from_err()
    }


    /*
    /// Spawn a specific range of shards in this process.
    pub fn spawn_range(shards: [u64; 2], total_shards: i32) {

    }
    */

}