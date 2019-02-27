use std::net::SocketAddr;

use futures::{future::Future, Stream};
use lapin_futures::{
    channel::{Channel, QueueDeclareOptions},
    client::{Client as AmpqClient, ConnectionOptions},
    types::FieldTable,
};
use lapin_futures::channel::BasicConsumeOptions;
use tokio::net::TcpStream;

use crate::errors::Error;

pub struct MessageBroker {
    client: Option<AmpqClient<TcpStream>>,
    channel: Option<Channel<TcpStream>>,
    group: String,
    subgroup: String
}

impl MessageBroker {
    fn new(group: String, subgroup: String) -> MessageBroker {
        Self {
            channel: None,
            client: None,
            group,
            subgroup
        }
    }

    fn connect(&mut self, addr: &SocketAddr) -> impl Future<Item = (), Error = Error> {
        TcpStream::connect(addr).map_err(Error::from).and_then(|stream| {
            AmpqClient::connect(stream, ConnectionOptions::default())
                .map_err(Error::from)
        }).and_then(|(ampq, heartbeat)| {
            tokio::spawn(heartbeat.map_err(|_| ()));
            ampq.create_channel().map_err(Error::from)
        }).and_then(|channel| {
            info!("Created AMPQ Channel With ID: {}", &channel.id);
            self.channel = Some(channel);
        })
    }

    fn subscribe(&self, evt: String) {
        let queue_name = format!("{}{}{}", self.group, self.subgroup, evt);
        let chan = match &self.channel {
            Some(c) => c,
            None => {}
        };
        chan.queue_declare(queue_name.as_str(), QueueDeclareOptions::default(), FieldTable::new())
            .and_then(|queue| {
                info!("Channel ID: {} has declared queue: {}", &chan.id, &queue_name);
                chan.basic_consume(&queue, "", BasicConsumeOptions::default(), FieldTable::new())
            })
            .and_then(|stream| {
                info!("Consumer Stream Received.");
                stream.for_each(move |message| {
                    debug!("Received Message: {:?}", message);
                    // TODO: Actually handle Message from consumer.
                    let decoded = std::str::from_utf8(&message.data).unwrap();
                    chan.basic_ack(message.delivery_tag, false);
                })
            }).map_err(Error::from)
    }
}