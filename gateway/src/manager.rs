use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::Arc,
    time::Duration,
    time::Instant
};

use futures::{
    future::Future,
    Stream,
    sync::mpsc::{
        channel, Receiver as MpscReceiver, Sender as MpscSender, unbounded,
        UnboundedReceiver, UnboundedSender
    }
};
use parking_lot::{Mutex, RwLock};
use tokio::runtime::current_thread;
use tokio::timer::Delay;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

use spectacles_model::gateway::{GatewayBot, GatewayEvent, Opcodes};

use crate::{
    constants::API_BASE,
    errors::Error,
    queue::{MessageSink, MessageSinkError, ReconnectQueue, ShardQueue},
    shard::{Shard, ShardAction}
};
use crate::EventHandler;
use crate::ManagerOptions;

/// The strategy in which you would like to spawn shards.
#[derive(Clone)]
pub enum ShardStrategy {
    /// The spawner will automatically spawn shards based on the amount recommended by Discord.
    Recommended,
    /// Spawns shards according to the amount specified, starting from shard 0.
    SpawnAmount(usize)
}

pub struct ShardMessage {

}

/// A collection of shards, keyed by their ID.
pub type ShardMap = HashMap<usize, Arc<Mutex<Shard>>>;
pub type MessageStream = UnboundedReceiver<(Arc<Mutex<Shard>>, TungsteniteMessage)>;
/// An organized group of Discord gateway shards.
pub struct ShardManager<H: EventHandler + Send + Sync + 'static> {
    /// The token that this shard uses to connect to the Discord API.
    pub token: String,
    /// The options used to configure this shard manager.
    pub options: Option<ManagerOptions<H>>,
    /// A collection of shards that have been spawned.
    pub shards: Arc<RwLock<ShardMap>>,
    _spawn_amount: usize,
    queue_sender: MpscSender<usize>,
    queue_receiver: Option<MpscReceiver<usize>>,
    reconnect_queue: ShardQueue,
    message_stream: Option<MessageStream>,
}

impl <H: EventHandler + Send + Sync + 'static> ShardManager<H> {
    /// Creates a new cluster, with the provided Discord API token.
    pub fn new(token: String, options: ManagerOptions<H>) -> impl Future<Item = ShardManager<H>, Error = Error> {
        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };

        let (queue_sender, queue_receiver) = channel(0);
        use reqwest::r#async::Client;
        Client::new().get(format!("{}/gateway/bot", API_BASE).as_str())
            .header("Authorization", token.clone()).send()
            .and_then(|mut resp| resp.json::<GatewayBot>())
            .map_err(Error::from)
            .map(|gb| {
                let shard_count = match options.strategy {
                    ShardStrategy::Recommended => gb.shards,
                    ShardStrategy::SpawnAmount(int) => int
                };
                Self {
                    token,
                    reconnect_queue: ShardQueue::new(shard_count),
                    queue_sender,
                    options: Some(options),
                    queue_receiver: Some(queue_receiver),
                    message_stream: None,
                    _spawn_amount: shard_count,
                    shards: Arc::new(RwLock::new(HashMap::new())),
                }
        })
    }

    /// Spawns shards up to the specified amount and identifies them with Discord.
    pub fn begin_spawn(mut self) {
        info!("Bootstrapping ShardManager.");
        let shard_count = self._spawn_amount.clone();
        debug!("Attempting to spawn {} shards.", &shard_count);
        for i in 0..shard_count {
            trace!("Sending shard {} to queue.", &i);
            tokio::spawn(self.reconnect_queue.push_back(i)
                .map_err(move |_| error!("Failed to place Shard {} into reconnect queue.", i))
            );
        }

        let (sender, receiver) = unbounded();
        self.message_stream = Some(receiver);
        let mut queue_sender = self.queue_sender.clone();
        let receiver = self.queue_receiver.take().unwrap();
        let token = self.token.clone();
        let newsender = sender.clone();
        let shards = self.shards.clone();
        let message_stream = self.message_stream.take().unwrap();
        let opts = self.options.take().unwrap();

        current_thread::spawn(self.reconnect_queue.pop_front()
            .and_then(move |shard| {
                let shard = shard.expect("Shard queue is empty.");
                queue_sender.try_send(shard).expect("Failed to send starting shard.");
                futures::future::ok(())
            })
            .map_err(|_| error!("Failed to pop shard in reconnect queue."))
            .and_then(move |_| receive_chan(
                receiver,
                token,
                shard_count,
                newsender,
                shards,
            ))
        );

        tokio::spawn(message_stream.for_each(move |(mut shard, message)| {
            let current_shard = shard.borrow_mut();
            let mut shard = current_shard.lock().clone();
            trace!("Websocket message received: {:?}", &message.clone());
            let event = shard.resolve_packet(&message).expect("Failed to parse the shard message.");
            if let Opcodes::Dispatch = event.op {
                tokio::spawn({
                    opts.handler.on_packet(&mut shard, event.clone());
                    futures::future::ok(())
                });
            }
            let action = shard.fulfill_gateway(event.clone());
            match action {
                Ok(a) => match a {
                    ShardAction::Autoreconnect => {
                        current_thread::spawn(shard.autoreconnect().map_err({
                            let shard = shard.info[0].clone();
                            move |err| {
                                error!("Shard {} failed to autoreconnect. {}", shard, err);
                            }
                        }));
                    },
                    ShardAction::Identify => {
                        debug!("[Shard {}] Identifying with the gateway.", &shard.info[0]);
                        if let Err(e) = shard.identify() {
                            warn!("[Shard {}] Failed to identify with gateway. {:?}", &shard.info[0], e);
                        };
                    },
                    ShardAction::Reconnect => {
                        shard.reconnect();
                        info!("[Shard {}] Reconnection successful.", &shard.info[0]);
                    },
                    ShardAction::Resume => {
                        shard.resume();
                        info!("[Shard {}] Successfully resumed session.", &shard.info[0]);
                    },
                    _ => {}
                },
                Err(e) => {
                    error!("Failed to perform action for Shard {}. {}", &shard.info[0], e);
                },
            };
            if let Some(GatewayEvent::READY) = event.t {
                &(self).queue();
                tokio::spawn({
                    opts.handler.on_shard_ready(&mut shard);
                    futures::future::ok(())
                });
            };

            futures::future::ok(())
        }));
    }

    fn queue(&mut self) {
        current_thread::spawn({
            self.reconnect_queue.pop_front()
                .and_then({
                    let mut sender = self.queue_sender.clone();
                    move |id| {
                        if let Some(next) = id {
                            if let Err(e) = sender.try_send(next) {
                                error!("Could not place shard ID in queue. {:?}", e);
                            }
                        }

                        futures::future::ok(())
                    }
                })
        });
    }


    /*
    /// Spawn a specific range of shards in this process.
    pub fn spawn_range(shards: [u64; 2], total_shards: i32) {

    }
    */

}
fn receive_chan(
    receiver: MpscReceiver<usize>,
    token: String,
    shardcount: usize,
    sender: UnboundedSender<(Arc<Mutex<Shard>>, TungsteniteMessage)>,
    shardmap: Arc<RwLock<ShardMap>>
) -> impl Future<Item = (), Error = ()> {
    receiver.for_each(move |shard_id| {
        debug!("Received notification to shart Shard {}.", shard_id);
        let shardmap = shardmap.clone();
        let token = token.clone();
        let sender = sender.clone();
        Delay::new(Instant::now() + Duration::from_secs(5))
            .map_err(|err| error!("Failed to pause before spawning the next shard. {:?}", err))
            .and_then(move |_| {
                current_thread::spawn(create_shard(token, [shard_id, shardcount], sender)
                    .map(move |shard| {
                        shardmap.write().insert(shard_id, shard);
                        info!("Shard {} has been successfully spawned.", shard_id);
                    })
                );
                futures::future::ok(())
            })
    }).map_err(|_| ())
}

fn create_shard(
    token: String,
    info: [usize; 2],
    sender: UnboundedSender<(Arc<Mutex<Shard>>, TungsteniteMessage)>,
) -> impl Future<Item = Arc<Mutex<Shard>>, Error = ()> {
    Shard::new(token.clone(), info)
        .map_err(move |err| error!("Failed to start Shard {}. {:?}", info[0], err))
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
                .map_err(|e| error!("Failed to forward shard messages to the sink. {:?}", e)))
            );
            futures::future::ok(shard)
        })
}
