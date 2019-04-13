use std::{
    env,
    fs
};
use std::sync::Arc;

use clap::ArgMatches;
use futures::future::Future;
use serde_json::value::RawValue;
use tokio::prelude::*;

use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};
use spectacles_gateway::{ShardManager, ShardStrategy};
use spectacles_model::gateway::{RequestGuildMembers, SendPacket, UpdateStatus, UpdateVoiceState};
use spectacles_model::snowflake::Snowflake;

use crate::errors::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct SpawnerOptions {
    amqp_url: String,
    amqp_subgroup: Option<String>,
    amqp_group: String,
    shard_count: Option<usize>,
    token: String,
    config_path: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecGatewayMessage<'a> {
    pub guild_id: Snowflake,
    #[serde(borrow)]
    pub packet: &'a RawValue,
}


pub fn start_sharder(config: SpawnerOptions) -> impl Future<Item=(), Error=Error> {
    let amqp_url = config.amqp_url.clone();
    let group = config.amqp_group.clone();
    let subgroup = config.amqp_subgroup.clone();
    let token = config.token.clone();
    let shard_count = match config.shard_count {
        Some(r) => ShardStrategy::SpawnAmount(r),
        None => ShardStrategy::Recommended
    };
    let amqp = AmqpBroker::new(amqp_url, group, subgroup).from_err();
    let sharder = ShardManager::new(token, shard_count).from_err();

    amqp.join(sharder).map(|(broker, mut manager)| {
        info!("Sharder has completed bootstrap - spawning shards.");
        let broker = Arc::new(broker);
        let (spawner, events) = manager.start_spawn();

        let broker_1 = Arc::clone(&broker);
        tokio::spawn(spawner.for_each(move |shard| {
            info!("Shard {:?} has successfully spawned.", shard.lock().info);
            let shard_num = shard.lock().info[0].to_string();

            broker_1.consume(&shard_num).for_each(move |message| {
                let status = serde_json::from_slice::<SendPacket<UpdateStatus>>(&message);
                let guild_members = serde_json::from_slice::<SendPacket<RequestGuildMembers>>(&message);
                let voice_state = serde_json::from_slice::<SendPacket<UpdateVoiceState>>(&message);

                if let Ok(packet) = status {
                    shard.lock().send_payload(packet.d).unwrap_or_else(|err| {
                        error!("Failed to send packet to the gateway. {:?}", err);
                    });
                } else if let Ok(packet) = guild_members {
                    shard.lock().send_payload(packet.d).unwrap_or_else(|err| {
                        error!("Failed to send packet to the gateway. {:?}", err);
                    });
                } else if let Ok(packet) = voice_state {
                    shard.lock().send_payload(packet.d).unwrap_or_else(|err| {
                        error!("Failed to send packet to the gateway. {:?}", err);
                    });
                };

                Ok(())
            })
        }));

        let broker_2 = Arc::clone(&broker);
        tokio::spawn(events.for_each(move |event| {
            let props = AmqpProperties::default().with_content_type("application/json".to_string());
            if let Some(name) = event.packet.t {
                let payload = event.packet.d.get().as_bytes().to_vec();
                let event = name.to_string();

                tokio::spawn(broker_2.publish(&event, payload, props.clone())
                    .map(|_| ())
                    .map_err(|err| {
                        error!("Failed to publish shard event to AMQP: {:?}", err);
                    })
                );
            };

            Ok(())
        }));

        let broker_3 = Arc::clone(&broker);
        let shard_count = manager.total_shards as u64;
        tokio::spawn(broker_3.consume("SEND").for_each(move |payload| {
            let message: SpecGatewayMessage = serde_json::from_slice(&payload)
                .expect("Failed to deserialize gateway message");
            let shard_id = (message.guild_id.0 >> 22) % shard_count;
            let shard_str = shard_id.to_string();
            let json = message.packet.get().as_bytes().to_vec();
            let props = AmqpProperties::default().with_content_type("application/json".to_string());

            broker_3.publish(&shard_str, json, props).map(|_| ()).map_err(|err| {
                error!("Failed to publish shard message - {:?}", err);
            })
        }));
    })
}

pub fn parse_args(results: &ArgMatches) -> Result<()> {
    let cfg = if results.value_of("config_path").is_some() || env::var("CONFIG_FILE_PATH").is_ok() {
        let path = results.value_of("CONFIG_PATH")
            .map(|s| s.to_string())
            .unwrap_or(env::var("CONFIG_FILE_PATH").unwrap());
        parse_config_file(path.to_string())?
    } else {
        parse_argv(results)?
    };

    let future = start_sharder(cfg);
    tokio::run(future.map_err(|err| {
        error!("An error occured while sharding. {:?}", err);
    }));

    Ok(())
}

fn parse_argv(results: &ArgMatches) -> Result<SpawnerOptions> {
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

fn parse_config_file(path: String) -> Result<SpawnerOptions> {
    let file = fs::read_to_string(path)?;

    if file.ends_with(".json") {
        Ok(serde_json::from_str::<SpawnerOptions>(&file)?)
    } else if file.ends_with(".toml") {
        Ok(toml::from_str::<SpawnerOptions>(&file)?)
    } else {
        Err(Error::InvalidFile)
    }
}
