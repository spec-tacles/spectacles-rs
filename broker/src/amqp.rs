use std::net::SocketAddr;

use futures::{future::Future, Stream};
use lapin_futures::{
    channel::{BasicConsumeOptions, BasicProperties, BasicPublishOptions, Channel, QueueDeclareOptions},
    client::{Client as AmqpClient, ConnectionOptions},
    types::FieldTable,
};
use tokio::net::TcpStream;

use crate::errors::Error;

/// Event handler for receiving messages from the message broker.
pub trait EventHandler {
    fn on_message(&self, payload: String);
}
/// Central message broker client.
pub struct MessageBroker {
    channel: Channel<TcpStream>,
    event_cb: Box<dyn EventHandler>,
    group: String,
    subgroup: String
}

impl MessageBroker {
    /// Creates a new message broker, with the provided address, groups. You must also provide a callback that will be called each time a message is received.
    pub fn new(addr: &SocketAddr, group: &'static str, subgroup: &'static str, event_cb: Box<dyn EventHandler>) -> impl Future<Item = MessageBroker, Error = Error> {
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
                event_cb,
                group: group.to_string(),
                subgroup: subgroup.to_string()
            }
        })

    }

    /// Closes the currently open channel.
    pub fn close(&self, code: u16, msg: String) -> impl Future<Item = (), Error = Error> {
        self.channel.close(code, msg.as_ref()).map_err(Error::from)
    }

    /// Publishes a payload for the provided event to the message broker.
    pub fn publish(&self, evt: &'static str, payload: Vec<u8>) -> impl Future<Item = Option<u64>, Error = Error> {
        info!("Publishing Event: {} to the Message broker.", evt);
        self.channel.basic_publish(
            self.group.as_ref(),
            evt,
            payload,
            BasicPublishOptions::default(),
            BasicProperties::default().with_content_type("application/json".to_string())
        ).map_err(Error::from)
    }

    /// Subscribes to the provided event
    pub fn subscribe(&self, evt: &'static str) -> impl Future<Item = (), Error = Error> + '_ {
        let queue_name = format!("{}{}{}", self.group, self.subgroup, evt);
        self.channel.queue_declare(queue_name.as_str(), QueueDeclareOptions::default(), FieldTable::new())
            .and_then(move |queue| {
                info!("Channel ID: {} has declared queue: {}", self.channel.id, queue_name);
                self.channel.basic_consume(&queue, "", BasicConsumeOptions::default(), FieldTable::new())
            })
            .and_then(move |stream| {
                info!("Consumer Stream Received.");
                stream.for_each(move |message| {
                    debug!("Received Message: {:?}", message);
                    let decoded = std::str::from_utf8(&message.data).unwrap();
                    self.event_cb.on_message(decoded.to_string());
                    self.channel.basic_ack(message.delivery_tag, false)
                })
            })
            .map_err(Error::from)
    }
}