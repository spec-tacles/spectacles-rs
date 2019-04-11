# Spectacles Client
A standalone application for handling specific Spectacles tasks.

## Current Features
- Spawning Discord shards.
- Publishing Discord events to a message broker (AMQP).
- HTTP rate limiter proxy.

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
spectacles shard -c 5 -g gateway -t YOURTOKEN HERE -u 127.0.0.1:5672
```

You may also provide the following environment variables.

`AMQP_URL`: The URL of the AMQP server that you would like to connect to.

`AMQP_GROUP`: The AMQP group (exchange) that will be used to register queues for Discord Events.

`AMQP_SUBGROUP`: The AMQP subgroup (exchange) that will be used to register queues for Discord Events.

`DISCORD_TOKEN`: The token of the bot that you wish to use to spawn shards.

`SHARD_COUNT`: The amount of shards to spawn.

## HTTP Rate Limiting
Included in the client binary is an HTTP rate limiter proxy.
Its goal is to forward incoming requests to the Discord API, all while gracefully handling rate limits.
This alleviates the need for a stateful rate limiting system in each micro-service which relies on the Discord API.

```
spectacles-ratelimit 
Starts an HTTP ratelimiter proxy, which preemptively ratelimits a service's requests to the Discord API.

USAGE:
    spectacles ratelimit [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config-path <PATH>    The location of the configuration file that you would like to use. Supports TOML and
                                JSON.
    -a, --address <ADDRESS>     The TCP address on which to listen for requests.

```

The following environment variables are supported.

`SERVER_ADDR`: The Socket address in which to listen the HTTP server on.