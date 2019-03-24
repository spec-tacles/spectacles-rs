use futures::{future::Future, Stream};
// use futures_backoff::Strategy;
use lapin_futures_native_tls::{AMQPConnectionNativeTlsExt, AMQPStream};
use lapin_futures_native_tls::lapin::{
    channel::{
        BasicConsumeOptions,
        BasicPublishOptions,
        Channel,
        ExchangeDeclareOptions,
        QueueBindOptions,
        QueueDeclareOptions
    },
    types::FieldTable,
};
use lapin_futures_native_tls::lapin::channel::BasicProperties;

use crate::errors::Error;

pub type AmqpProperties = BasicProperties;

/// Central AMQP message brokers client.
#[derive(Clone)]
pub struct AmqpBroker {
    /// The AMQP channel used for processing messages.
    pub channel: Channel<AMQPStream>,
    /// The group used for consuming and producing messages.
    pub group: String,
    /// The subgroup used for consuming and producing messages.
    pub subgroup: Option<String>
}

impl AmqpBroker {
    /// Creates a new AMQP-based message broker, with the provided address, and groups.
    /// # Example
    /// ```rust,norun
    /// use std::env::var;
    /// use spectacles_brokers::amqp::*;
    /// use futures::future::future;
    ///
    /// fn main() {
    ///     let amqp = var("AMQP_URL").expect("No AMQP Address has been provided.");
    ///     tokio::run({
    ///         AmqpBroker::new(&amqp, "mygroup".to_string(), None)
    ///         .map(|broker| {
    ///             /// Publish and subscribe to events here.
    ///         });
    ///     });
    /// }
    /// ```

    pub fn new<'a>(amqp_uri: &str, group: String, subgroup: Option<String>) -> impl Future<Item=AmqpBroker, Error=Error> + 'a {
        /*let retry_strategy = Strategy::fibonacci(Duration::from_secs(2))
            .with_max_retries(10);*/
        let gr = group.clone();
        amqp_uri.connect_cancellable(|err| {
            eprintln!("Error encountered while attempting heartbeat. {}", err);
        }).map_err(Error::from)
            .and_then(|(amqp, _)| amqp.create_channel().from_err())
            .and_then(move |channel| {
            debug!("Created AMQP Channel With ID: {}", &channel.id);
                channel.exchange_declare(&gr, "direct", ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
                }, FieldTable::new()).map(|_| channel).from_err()
            }).map(|channel| {
            Self {
                channel,
                group,
                subgroup,
            }
        }).from_err()
    }

    /// Closes the currently open channel.
    pub fn close(&self, code: u16, msg: String) -> impl Future<Item = (), Error = Error> {
        self.channel.close(code, msg.as_ref()).map_err(Error::from)
    }

    /// Publishes a payload for the provided event to the message brokers.
    /// You must serialize all payloads to a Vector of bytes.
    /// This method accepts an AMQPProperties struct which will set the AMQP properties for this message.
    /// See [here](https://docs.rs/amq-protocol/1.2.0/amq_protocol/protocol/basic/struct.AMQPProperties.html) for more details on the various AMQP properties.
    ///
    /// # Example
    /// ```rust,norun
    /// AmqpBroker::new(AMQP_URI, "mygroup".to_string(), None)
    ///    .and_then(|broker| {
    ///         broker.publish(
    ///             "MESSAGE_CREATE",
    ///             "{"content": "Hi"}".as_bytes().to_vec(),
    ///             AmqpProperties::default().with_content_type("application/json")
    ///         )
    ///     })
    /// ```
    ///
    pub fn publish(&self, evt: &str, payload: Vec<u8>, properties: AmqpProperties) -> impl Future<Item=Option<u64>, Error=Error> {
        debug!("Publishing event: {} to the AMQP server.", evt);
        self.channel.basic_publish(
            self.group.as_ref(),
            evt,
            payload,
            BasicPublishOptions::default(),
            properties
        ).map_err(Error::from)
    }

    /// Subscribes to the provided event, with a callback that is called when an event is received.
    /// # Example
    /// ```rust,norun
    /// AmqpBroker::new(&addr, "mygroup", None)
    ///    .and_then(|broker| {
    ///         broker.subscribe("MESSAGE_CREATE", |payload| {
    ///             println!("Message Event Received: {}", payload);
    ///         })
    ///     })
    ///     .map(|_| {
    ///         println!("Successfully subscribed to the group!");
    ///     })
    /// ```
    ///
    pub fn subscribe<C>(self, evt: String, callback: C) -> impl Future<Item=(), Error=Error>
        where C: Fn(&str) + Send + Sync + 'static
    {
        let queue_name = match &self.subgroup {
            Some(g) => format!("{}:{}:{}", self.group, g, evt),
            None => format!("{}:{}", self.group, evt)
        };
        let channel = self.channel.clone();
        let group = self.group.clone();
        channel.queue_declare(
            queue_name.as_str(),
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::new()
        ).and_then(move |queue| {
            debug!("Channel ID: {} has declared queue: {}", channel.id, queue_name);
            channel.queue_bind(
                queue_name.as_str(),
                &group,
                evt.as_str(),
                QueueBindOptions::default(),
                FieldTable::new(),
            ).and_then(move |_| channel.basic_consume(
                &queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::new(),
            ).and_then(move |stream| stream.for_each(move |message| {
                debug!("Incoming message received from AMQP with a delivery tag of {}.", &message.delivery_tag);
                let decoded = std::str::from_utf8(&message.data).unwrap();
                tokio::spawn({
                    callback(decoded);
                    futures::future::ok(())
                });
                channel.basic_ack(message.delivery_tag, false)
            })))
        }).map_err(Error::from)
    }
}
