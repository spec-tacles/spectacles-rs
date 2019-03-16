use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use futures::{future::Future, Stream};
use futures_backoff::Strategy;
use lapin_futures::{
    channel::{
        BasicConsumeOptions,
        BasicProperties,
        BasicPublishOptions,
        Channel,
        ExchangeDeclareOptions,
        QueueBindOptions,
        QueueDeclareOptions
    },
    client::{Client as AmqpClient, ConnectionOptions},
    types::FieldTable,
};
use tokio::net::TcpStream;

use crate::errors::Error;

/// Central AMQP message brokers client.
#[derive(Clone)]
pub struct AmqpBroker {
    /// The AMQP channel used for processing messages.
    pub channel: Arc<Channel<TcpStream>>,
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
    /// use spectacles_brokers::AmqpBroker;
    /// use std::net::SocketAddr;
    /// use futures::future::future;
    ///
    /// fn main() {
    ///     let addr = var("AMQP_ADDR").expect("No AMQP Address has been provided.");
    ///     let addr: SocketAddr = addr.parse().expect("Malformed URL provided.");
    ///     tokio::run({
    ///         AmqpBroker::new(&addr, "mygroup".to_string(), None)
    ///         .map(|broker| {
    ///             /// Publish and subscribe to events here.
    ///         });
    ///     });
    /// }
    /// ```

    pub fn new<'a>(addr: &SocketAddr, group: String, subgroup: Option<String>) -> impl Future<Item = AmqpBroker, Error = Error> + 'a {
        let retry_strategy = Strategy::fibonacci(Duration::from_secs(2))
            .with_max_retries(10);
        let (amqp, heartbeat) = retry_strategy.retry(|| {
            TcpStream::connect(addr).map_err(Error::from).and_then(|stream| {
                AmqpClient::connect(stream, ConnectionOptions::default())
                    .map_err(Error::from)
            })
        }).wait().unwrap();
        amqp.create_channel().map_err(Error::from).and_then(move |channel| {
            tokio::spawn(heartbeat.map_err(|_| ()));
            debug!("Created AMQP Channel With ID: {}", &channel.id);
            channel.exchange_declare(group.as_ref(), "direct", ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            }, FieldTable::new()).map(move |_| {
                Self {
                    channel: Arc::new(channel),
                    group,
                    subgroup
                }
            }).map_err(Error::from)
        })
    }

    /// Closes the currently open channel.
    pub fn close(&self, code: u16, msg: String) -> impl Future<Item = (), Error = Error> {
        self.channel.close(code, msg.as_ref()).map_err(Error::from)
    }

    /// Publishes a payload for the provided event to the message brokers.
    /// You must serialize all payloads to a Vector of bytes.
    /// # Example
    /// ```rust,norun
    /// AmqpBroker::new(&addr, "mygroup", None)
    ///    .and_then(|broker| {
    ///         broker.publish("MESSAGE_CREATE", "{"content": "Hi"}".as_bytes().to_vec());
    ///     })
    /// ```
    ///
    pub fn publish(&self, evt: &str, payload: Vec<u8>) -> impl Future<Item = Option<u64>, Error = Error> {
        debug!("Publishing event: {} to the AMQP server.", evt);
        self.channel.basic_publish(
            self.group.as_ref(),
            evt,
            payload,
            BasicPublishOptions::default(),
            BasicProperties::default().with_content_type("application/json".to_string())
        ).map_err(Error::from)
    }

    /// Subscribes to the provided event, with a callback that is called when an event is received.
    /// NOTE: Do not forget that you must manually acknowledge messages, as shown in the example below.
    /// # Example
    /// ```rust,norun
    /// AmqpBroker::new(&addr, "mygroup", None)
    ///    .map(|broker| {
    ///         broker.subscribe("MESSAGE_CREATE", |(message, ack)| {
    ///             tokio::spawn(ack);
    ///             println!("Message Event Received: {}");
    ///         });
    ///     })
    /// ```
    ///
    pub fn subscribe<C>(self, evt: &'static str, callback: C) -> Self
        where C: Fn(String) + Send + Sync + 'static
    {
        let queue_name = match &self.subgroup {
            Some(g) => format!("{}:{}:{}", self.group, g, evt),
            None => format!("{}:{}", self.group, evt)
        };
        let channel = Arc::clone(&self.channel);
        let future = channel.queue_declare(
            queue_name.as_str(),
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::new()
        ).and_then({
            let channel = Arc::clone(&self.channel);
            let group = self.group.clone();
            move |queue| {
                debug!("Channel ID: {} has declared queue: {}", channel.id, queue_name);
                channel.queue_bind(
                    queue_name.as_str(),
                    group.as_ref(),
                    evt,
                    QueueBindOptions::default(),
                    FieldTable::new()
                ).and_then(move  |_| channel.basic_consume(&queue, "", BasicConsumeOptions::default(), FieldTable::new()))
            }
        }).and_then({
            let channel = Arc::clone(&self.channel);
            move |stream| stream.for_each(move |message| {
                debug!("Incoming message received from AMQP with a delivery tag of {}.", &message.delivery_tag);
                let decoded = String::from_utf8(message.data.to_owned()).unwrap();
                callback(decoded);
                channel.basic_ack(message.delivery_tag, false)
            })
        }).map_err(Error::from);
        tokio::spawn(future.map_err(move |err| {
            error!("Error encountered on event: {} - {}", evt, err);
        }));

        self
    }
}