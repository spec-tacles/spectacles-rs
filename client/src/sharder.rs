use std::{
    env,
    fs
};
use std::sync::Arc;

use clap::ArgMatches;
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

pub async fn start_sharder(config: SpawnerOptions) -> Result<()> {
    let amqp_url = config.amqp_url.clone();
    let group = config.amqp_group.clone();
    let subgroup = config.amqp_subgroup.clone();
    let token = config.token.clone();
    let shard_count = match config.shard_count {
        Some(r) => ShardStrategy::SpawnAmount(r),
        None => ShardStrategy::Recommended
    };
    let amqp = Arc::new(await!(AmqpBroker::new(&amqp_url, group, subgroup))?);
    let mut manager = await!(ShardManager::new(token, shard_count))?;
    let (mut spawner, mut packets) = manager.begin_spawn();

    // Handle the SEND queue, where shard ID can be calculated from guild ID and forwarded to the correct shard queue.
    let mut send_consumer = await!(amqp.consume("SEND")).expect("Failed to consume SEND event");
    let shard_count = manager.total_shards as u64;
    let broker = Arc::clone(&amqp);

    tokio::spawn_async(async move {
        while let Some(Ok(message)) = await!(send_consumer.next()) {
            let message: SpecGatewayMessage = serde_json::from_slice(&message)
                .expect("Failed to deserialize gateway message");
            let shard_id = (message.guild_id.0 >> 22) % shard_count;
            let shard_str = shard_id.to_string();
            let json = message.packet.get().as_bytes().to_vec();
            let props = AmqpProperties::default().with_content_type("application/json".to_string());
            if let Err(e) = await!(broker.publish(&shard_str, json, props)) {
                error!("Failed to forward packet to Shard {} - {}", shard_str, e);
            };
        }
    });

    // Handle the queue for each shard ID as the shard is spawned.
    // Upon consuming an event, the shard will send the packet to the Discord gateway.
    let broker = Arc::clone(&amqp);
    tokio::spawn_async(async move {
        while let Some(Ok(shard)) = await!(spawner.next()) {
            let shard_num = shard.lock().info[0].to_string();
            let mut consumer = await!(broker.consume(&shard_num)).expect("Failed to consume shard events");
            while let Some(Ok(message)) = await!(consumer.next()) {
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
            }
        }
    });

    // Handle shard event stream as each event arrives, and forwards it to the appropriate queue.
    let broker = Arc::clone(&amqp);
    tokio::spawn_async(async move {
        while let Some(Ok(event)) = await!(packets.next()) {
            let props = AmqpProperties::default().with_content_type("application/json".to_string());
            if let Some(name) = event.packet.t {
                let payload = event.packet.d.get().as_bytes().to_vec();
                let event = name.to_string();
                await!(broker.publish(&event, payload, props.clone()))
                    .expect("Failed to publish shard event to AMQP");
            };
        }
    });

    Ok(())
}

pub async fn parse_args(results: ArgMatches) -> Result<()> {
    let cfg = if results.value_of("config_path").is_some() || env::var("CONFIG_FILE_PATH").is_ok() {
        let path = results.value_of("CONFIG_PATH")
            .map(|s| s.to_string())
            .unwrap_or(env::var("CONFIG_FILE_PATH").unwrap());
        parse_config_file(path.to_string())?
    } else {
        parse_argv(results.clone())?
    };

    await!(start_sharder(cfg))?;

    Ok(())
}

fn parse_argv(results: ArgMatches) -> Result<SpawnerOptions> {
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
