# Spectacles Rust
A library for creating powerful, fast, and scalable Discord applications and services.


## Introduction
Spectacles is a service which allows for creating components which interact with the Discord API using a microservices-like architecture.
All components are usually unified through a message brokering system, which means they can be fully distributed (spread out acress servers).

Imagine the following: You want to push a major update in your bot, but you fear the downtime that it would bring. With Spectacles, you can have your application split up into "workers", each of whom will consume Discord events from the message broker. So if you take down one worker to update it, the other worker can still receive events, thus acheiving zero downtime.

The microservices architecture is also very beneficial in the sense that you can scale your bot's components with ease. If your two workers are receiving a lot of load, simply add a third worker, for improved load balancing.

## Getting started
This library features seveal important crates to help you get started.

[Spectacles Client](client/README.md) - A standalone binary for Spectacles, which includes a Discord gateway and an event publishing system.

[Spectacles Gateway](gateway/README.md) - A Spectacles gateway implementation for Discord with enables stateless sharding.

[Spectacles Models](models/README.md) - A collection of data types than are used for serializing and deserializing data from the Discord API.

[Spectacles REST](rest/README.md) - A Spectacles implementation of Discord's REST API methods.

See each crate for more details on how to get started with your application.