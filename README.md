# Spectacles Rust
A library for creating powerful, fast, and scalable Discord applications and services.


## Introduction
Spectacles is a service which allows for creating components which interact with the Discord API using a microservices-like architecture.
All components are usually unified through a message brokering system, which means they can be fully distributed (spread out across servers).

Imagine the following: You want to push a major update in your bot, but you fear the downtime that it would bring. With Spectacles, you can have your application split up into "workers", each of whom will consume Discord events from the message broker. So if you take down one worker to update it, the other worker can still receive events, thus achieving zero downtime.

The microservices architecture is also very beneficial in the sense that you can scale your bot's components with ease. If your two workers are receiving a lot of load, simply add a third worker, for improved load balancing.
If you so choose, you may even have your bot use different programming languages for each service. For example, you could have your gateway in Rust, and your workers in Golang. They will all come together with help from the message broker.

## Getting started
This library features several important crates to help you get started.

[Spedctacles Brokers](brokers/) - Message brokers which allow for powerful communication between services.
[Spectacles Client](client/) - A standalone binary for Spectacles, which includes a Discord gateway and an event publishing system.

[Spectacles Gateway](gateway/) - A Spectacles gateway implementation for Discord with enables stateless sharding.

[Spectacles Models](models/) - A collection of data types than are used for serializing and deserializing data from the Discord API.

[Spectacles REST](rest/) - A Spectacles implementation of Discord's REST API methods.

See each crate for more details on how to get started with your application.