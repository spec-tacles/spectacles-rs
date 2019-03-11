use std::env;
use std::net::SocketAddr;

use clap::ArgMatches;
use futures::future::Future;
use tokio::runtime::current_thread;

use spectacles_brokers::AmqpBroker;
use spectacles_gateway::{EventHandler, ManagerOptions, Shard, ShardManager, ShardStrategy};
use spectacles_model::gateway::ReceivePacket;

use crate::errors::Error as MyError;

pub struct SpawnerConfig {
    amqp_url: SocketAddr,
    amqp_subgroup: Option<String>,
    amqp_group: String,
    shard_count: Option<usize>,
    token: String
}

pub struct Handler {
    broker: AmqpBroker
}

impl EventHandler for Handler {
    fn on_packet(&self, shard: &mut Shard, packet: ReceivePacket) {
        let info = shard.info.clone();
        if packet.t.is_none() { unimplemented!(); }
        else {
            let event = packet.t.unwrap().to_string();
            let broker = &self.broker;
            let payload = packet.d.get().as_bytes().to_vec();
            current_thread::spawn({
                broker.publish(event.as_ref(), payload)
                    .map(move |_| {
                        info!("Sent event: {} by Shard {} to AMQP.", event, info[0]);
                    })
                    .map_err(|err| {
                        error!("Failed to publish event to the AMQP broker. {}", err);
                    })
            });
        }
    }
}

pub fn start_sharder(config: SpawnerConfig) -> impl Future<Item = (), Error = MyError> {
    let amqp_url = config.amqp_url.clone();
    let group = config.amqp_group.clone();
    let subgroup = config.amqp_subgroup.clone();
    let token = config.token.clone();
    let shard_count = match config.shard_count {
        Some(r) => ShardStrategy::SpawnAmount(r),
        None => ShardStrategy::Recommended
    };
    let amqpconn = AmqpBroker::new(&amqp_url, group, subgroup);
    let sharder = amqpconn.map_err(MyError::from)
        .and_then(move |broker| {
            ShardManager::new(token, ManagerOptions {
                strategy: shard_count,
                handler: Handler { broker }
            }).map_err(MyError::from)
        });
    sharder.map(|manager| manager.begin_spawn())
        .map_err(MyError::from)
}

pub fn parse_args(results: &ArgMatches) -> Result<(), MyError> {
    let mut amqp_url;
    let mut amqp_group;
    let mut amqp_subgroup;
    let mut token;
    let shard_count;

    amqp_url = if let Some(r) = results.value_of("url") {
        r.to_string()
    } else {
        env::var("AMQP_URL").expect("No AMQP URL provided in arguments or ENV.")
    };
    amqp_group = if let Some(r) = results.value_of("group") {
        r.to_string()
    } else {
        env::var("AMQP_GROUP").expect("No AMQP URL provided in arguments or ENV.")
    };
    amqp_subgroup = if let Some(r) = results.value_of("subgroup") {
        Some(r.to_string())
    } else {
        match env::var("SHARD_COUNT") {
            Ok(e) => Some(e),
            Err(_) => None
        }
    };
    shard_count = if let Some(r) = results.value_of("count") {
        let res = r.to_string().parse::<usize>();
        if let Err(_) = res {
            panic!("Invalid integer for shard count provided, please try again.");
        };
        Some(res.unwrap())
    } else {
        match env::var("SHARD_COUNT") {
            Ok(e) => {
                let res = e.parse::<usize>();
                if let Err(_) = res {
                    panic!("Invalid integer for shard count provided, please try again.");
                };
                Some(res.unwrap())
            },
            Err(_) => None
        }
    };
    token = if let Some(r) = results.value_of("token") {
        r.to_string()
    } else {
        env::var("DISCORD_TOKEN").expect("No Discord token provided in arguments or ENV.")
    };

    let amqp_url: SocketAddr = amqp_url.parse().expect("Malformed AMQP URL provided, please check the URL and try again.");

    current_thread::run(start_sharder(SpawnerConfig {
        amqp_group,
        amqp_subgroup,
        amqp_url,
        shard_count,
        token
    }).map_err(|err| {
        error!("Failed to spawn Discord shards. {}", err);
    }));
    Ok(())
}