use std::net::SocketAddr;

use futures::{future::Future, Stream};
use lapin_futures::{
    channel::{Channel, QueueDeclareOptions},
    client::{Client as AmqpClient, ConnectionOptions},
    types::FieldTable,
};
use lapin_futures::channel::BasicConsumeOptions;
use tokio::net::TcpStream;

use crate::errors::Error;

pub struct MessageBroker {
    channel: Channel<TcpStream>,
    group: String,
    subgroup: String
}

impl MessageBroker {
    fn new(addr: &SocketAddr, group: String, subgroup: String) -> impl Future<Item = MessageBroker, Error = Error> {
        TcpStream::connect(addr).map_err(Error::from).and_then(|stream| {
            AmqpClient::connect(stream, ConnectionOptions::default())
                .map_err(Error::from)
        }).and_then(|(amqp, heartbeat)| {
            tokio::spawn(heartbeat.map_err(|_| ()));
            amqp.create_channel().map_err(Error::from)
        }).map(move |channel| {
            info!("Created AMQP Channel With ID: {}", &channel.id);
            Self {
                channel,
                group,
                subgroup
            }
        })

    }

    fn subscribe(&self, evt: String) -> impl Future<Item = (), Error = Error> {
        let queue_name = format!("{}{}{}", self.group, self.subgroup, evt);
        self.channel.queue_declare(queue_name.as_str(), QueueDeclareOptions::default(), FieldTable::new())
            .and_then(|queue| {
                info!("Channel ID: {} has declared queue: {}", self.channel.id, queue_name);
                self.channel.basic_consume(&queue, "", BasicConsumeOptions::default(), FieldTable::new())
            })
            .and_then(|stream| {
                info!("Consumer Stream Received.");
                stream.for_each(|message| {
                    debug!("Received Message: {:?}", message);
                    // TODO: Actually handle Message from consumer.
                    let decoded = std::str::from_utf8(&message.data).unwrap();
                    self.channel.basic_ack(message.delivery_tag, false)
                })
            }).map_err(Error::from)
    }
}