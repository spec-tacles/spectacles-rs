use std::{
    env,
    fs
};

use clap::ArgMatches;
use futures::future::Future;
use tokio::runtime::current_thread;

use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};
use spectacles_gateway::{EventHandler, ManagerOptions, Shard, ShardManager, ShardStrategy};
use spectacles_model::gateway::{ReceivePacket, RequestGuildMembers, UpdateStatus, UpdateVoiceState};

use crate::errors::{Error as MyError, Error};

#[derive(Serialize, Deserialize, Clone)]
pub struct SpawnerOptions {
    amqp_url: String,
    amqp_subgroup: Option<String>,
    amqp_group: String,
    shard_count: Option<usize>,
    token: String,
    config_path: Option<String>
}

pub struct Handler {
    broker: AmqpBroker
}

impl EventHandler for Handler {
    fn on_shard_ready(&self, shard: &mut Shard) {
        let broker = self.broker.clone();
        let shard_num = shard.info[0].clone().to_string();

        tokio::spawn(broker.subscribe(shard_num, {
            let shard = shard.clone();
            move |payload| {
                if let Ok(packet) = serde_json::from_str::<UpdateStatus>(payload) {
                    let _ = shard.send_payload(packet).map_err(|err| {
                        error!("Failed to send packet to the gateway. {:?}", err);
                    });
                };
                if let Ok(packet) = serde_json::from_str::<RequestGuildMembers>(payload) {
                    let _ = shard.send_payload(packet).map_err(|err| {
                        error!("Failed to send packet to the gateway. {:?}", err);
                    });
                };
                if let Ok(packet) = serde_json::from_str::<UpdateVoiceState>(payload) {
                    let _ = shard.send_payload(packet).map_err(|err| {
                        error!("Failed to send packet to the gateway. {:?}", err);
                    });
                };
            }
        }).map_err(|err| {
            error!("Failed to subscribe to the shard stream. {}", err);
        }));
    }

    fn on_packet(&self, shard: &mut Shard, packet: ReceivePacket) {
        let info = shard.info.clone();
        match packet.t {
            Some(event) => {
                let evt = event.to_string();
                let broker = &self.broker;
                let payload = packet.d.get().as_bytes().to_vec();
                current_thread::spawn({
                    broker.publish(
                        evt.as_ref(),
                        payload,
                        AmqpProperties::default().with_content_type("application/json".to_string()),
                    ).map(move |_| {
                        info!("Sent event: {} by Shard {} to AMQP.", event, info[0]);
                    }).map_err(|err| {
                        error!("Failed to publish event to the AMQP broker. {}", err);
                    })
                });
            },
            None => {}
        };

    }
}

pub fn start_sharder(config: SpawnerOptions) -> impl Future<Item=(), Error=MyError> {
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
        .from_err()
}

pub fn parse_args(results: &ArgMatches) -> Result<(), MyError> {
    let cfg = if results.value_of("config_path").is_some() || env::var("CONFIG_FILE_PATH").is_ok() {
        let path = results.value_of("CONFIG_PATH")
            .map(|s| s.to_string())
            .unwrap_or(env::var("CONFIG_FILE_PATH").unwrap());
        parse_config_file(path.to_string())?
    } else {
        parse_argv(results)?
    };

    current_thread::run(start_sharder(cfg).map_err(|err| {
        error!("Failed to spawn Discord shards. {}", err);
    }));

    Ok(())
}

fn parse_argv(results: &ArgMatches) -> Result<SpawnerOptions, Error> {
    let amqp_url = results.value_of("url").map(String::from)
        .unwrap_or(env::var("AMQP_URL").expect("No AMQP URL provided in arguments or ENV."));
    let amqp_group = results.value_of("group").map(String::from)
        .unwrap_or(env::var("AMQP_GROUP").expect("No AMQP URL provided in arguments or ENV."));
    let amqp_subgroup = results.value_of("subgroup").map(String::from)
        .map_or_else(|| {
            match env::var("SHARD_COUNT") {
                Ok(e) => Some(e),
                Err(_) => None
            }
        }, |e| Some(e));
    let shard_count = results.value_of("count")
        .map(|s| s.to_string().parse::<usize>())
        .map_or_else(|| {
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
        }, |u| Some(u.unwrap()));
    let token = results.value_of("token").map(String::from)
        .unwrap_or(env::var("DISCORD_TOKEN").expect("No Discord token provided in arguments or ENV."));

    Ok(SpawnerOptions {
        config_path: None,
        amqp_group,
        amqp_subgroup,
        amqp_url,
        shard_count,
        token
    })
}

fn parse_config_file(path: String) -> Result<SpawnerOptions, Error> {
    let file = fs::read_to_string(path)?;

    if file.ends_with(".json") {
        Ok(serde_json::from_str::<SpawnerOptions>(&file)?)
    } else if file.ends_with(".toml") {
        Ok(toml::from_str::<SpawnerOptions>(&file)?)
    } else {
        Err(Error::InvalidFile)
    }
}
