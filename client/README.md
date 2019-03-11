# Spectacles Client
A standalone application for handling specific Spectacles tasks.

## Current Features
- Spawning Discord shards.
- Publishing Discord events to a message broker (AMQP)
- (WIP) Ratelimiter Proxy.

## Sharding
This application features a built-in sharder which can publish all events received from the Discord API to a message broker.

```
spectacles-shard 
Spawn Discord shards and publish events to a message broker.

USAGE:
    spectacles shard [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <COUNT>          The amount of shards to spawn. If omitted, the recommended amount of shards will be
                                 spawned.
    -g, --group <GROUP>          The AMQP group (exchange) that will be used to register queues for Discord Events.
    -s, --subgroup <SUBGROUP>    The AMQP subgroup (exchange) that will be used to register queues for Discord Events.
    -t, --token <TOKEN>          The Discord token that will be used to connect to the gateway.
    -u, --amqpurl <URL>          The AMQP server to publish events to.

```

For example, to spawn 5 shards with an AMQP group, you may do:
```
spectacles shard -c 5 -g gateway -t YOURTOKEN HERE -u 127.0.0.0:5672
```

You may also provide the following environment variables.

`AMQP_URL`: The URL of the AMQP server that you would like to connect to.

`AMQP_GROUP`: The AMQP group (exchange) that will be used to register queues for Discord Events.

`AMQP_SUBGROUP`: The AMQP subgroup (exchange) that will be used to register queues for Discord Events.

`DISCORD_TOKEN`: The token of the bot that you wish to use to spawn shards.

`SHARD_COUNT`: The amount of shards to spawn.