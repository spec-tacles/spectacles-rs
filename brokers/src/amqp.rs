use std::net::SocketAddr;

use futures::{future::Future, Stream};
use lapin_futures::{
    channel::{BasicConsumeOptions, BasicProperties, BasicPublishOptions, Channel, QueueDeclareOptions},
    client::{Client as AmqpClient, ConnectionOptions},
    types::FieldTable,
};
use tokio::net::TcpStream;

use crate::{errors::Error, MessageHandler};

/// Central AMQP message brokers client.
#[derive(Clone)]
pub struct AmqpBroker<H: MessageHandler + Send + Sync> {
    /// The AMQP channel used for processing messages.
    pub channel: Channel<TcpStream>,
    event_cb: H,
    /// The group used for consuming and producing messages.
    pub group: String,
    /// The subgroup used for consuming and producting messages.
    pub subgroup: Option<String>
}

impl <H: MessageHandler + Send + Sync> AmqpBroker <H> {
    /// Creates a new AMQP-based message broker, with the provided address, groups, and a message handler struct.
    /// # Example
    /// ```rust,norun
    ///     use spectacles_brokers::{AmqpBroker, MessageHandler};
    ///
    ///     fn main() {
    ///         let addr = std::env::var("AMPQ_ADDR").expect("No AMQP Address detected");
    ///          let socketaddr: SocketAddr = addr.parse().expect("Failed to parse this AMQP Address.");
    ///          tokio::run({
    ///              AmqpBroker::new(&socketaddr, "gateway", None, MessageHandle)
    ///              .and_then(|broker| broker.subscribe("MESSAGE_CREATE"))
    ///              .and_then(|broker| broker.subscribe("GUILD_CREATE"))
    ///          })
    ///     }
    /// ```


    pub fn new(addr: &SocketAddr, group: &'static str, subgroup: Option<&'static str>, event_cb: H) -> impl Future<Item = AmqpBroker<H>, Error = Error> {
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
                subgroup: subgroup.map(|g| g.to_string())
            }
        })
    }

    /// Closes the currently open channel.
    pub fn close(&self, code: u16, msg: String) -> impl Future<Item = (), Error = Error> {
        self.channel.close(code, msg.as_ref()).map_err(Error::from)
    }

    /// Publishes a payload for the provided event to the message brokers.
    pub fn publish(&self, evt: &'static str, payload: Vec<u8>) -> impl Future<Item = Option<u64>, Error = Error> {
        info!("Publishing Event: {} to the Message brokers.", evt);
        self.channel.basic_publish(
            self.group.as_ref(),
            evt,
            payload,
            BasicPublishOptions::default(),
            BasicProperties::default().with_content_type("application/json".to_string())
        ).map_err(Error::from)
    }

    /// Subscribes to the provided event.
    pub fn subscribe(&self, evt: &'static str) -> impl Future<Item = &AmqpBroker<H>, Error = Error> {
        let queue_name = match &self.subgroup {
            Some(g) => format!("{}:{}:{}", self.group, g, evt),
            None => format!("{}:{}", self.group, evt)
        };

        self.channel.queue_declare(
            queue_name.as_str(),
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::new()
        ).and_then(move |queue| {
            info!("Channel ID: {} has declared queue: {}", self.channel.id, queue_name);
            self.channel.basic_consume(&queue, "", BasicConsumeOptions::default(), FieldTable::new())
        }).and_then(move |stream| {
            info!("Consumer Stream Received.");
            stream.for_each(move |message| {
                debug!("Received Message: {:?}", message);
                self.channel.basic_ack(message.delivery_tag, false);
                let decoded = std::str::from_utf8(&message.data).unwrap();
                self.event_cb.on_message(evt, decoded.to_string());
                Ok(())
            })
        }).map(move |_| self.clone()).map_err(Error::from)
    }
}