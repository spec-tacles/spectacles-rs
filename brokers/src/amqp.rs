use std::sync::Arc;
use std::time::Duration;

use futures::{future::Future, Stream};
use futures::sync::mpsc::{unbounded, UnboundedReceiver};
use futures_backoff::Strategy;
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
    client::{Client as LapinClient, HeartbeatHandle},
    types::FieldTable,
};
use lapin_futures_native_tls::lapin::channel::BasicProperties;
use tokio::prelude::*;

use crate::errors::Error;

pub type AmqpProperties = BasicProperties;

/// A stream of messages that are being consumed in the message queue.
pub struct AmqpConsumer {
    recv: UnboundedReceiver<Vec<u8>>
}

impl AmqpConsumer {
    fn new(recv: UnboundedReceiver<Vec<u8>>) -> Self {
        Self {
            recv
        }
    }
}

impl Stream for AmqpConsumer {
    type Item = Vec<u8>;
    type Error = ();
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.recv.poll()
    }
}


#[derive(Clone)]
struct ProducerState {
    connection: LapinClient<AMQPStream>,
    heartbeat: Arc<HeartbeatHandle>,
    channel: Channel<AMQPStream>,
}

#[derive(Clone)]
struct ConsumerState {
    connection: LapinClient<AMQPStream>,
    heartbeat: Arc<HeartbeatHandle>,
}

/// Central AMQP message brokers client.
#[derive(Clone)]
pub struct AmqpBroker {
    /// The group used for consuming and producing messages.
    pub group: String,
    /// The subgroup used for consuming and producing messages.
    pub subgroup: Option<String>,
    prod_state: ProducerState,
    consume_state: ConsumerState,
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
    ///         AmqpBroker::new(amqp, "mygroup".to_string(), None)
    ///         .map(|broker| {
    ///             /// Publish and subscribe to events here.
    ///         });
    ///     });
    /// }
    /// ```

    pub fn new(amqp_uri: String, group: String, subgroup: Option<String>) -> impl Future<Item=AmqpBroker, Error=Error> {
        let retry_strategy = Strategy::fibonacci(Duration::from_secs(2))
            .with_max_retries(10);
        let uri = amqp_uri.clone();
        let producer = retry_strategy.retry(move || uri.as_str().connect_cancellable(|err| {
            eprintln!("Error encountered while attempting heartbeat. {}", err);
        })).from_err::<Error>();
        let consumer = retry_strategy.retry(move || amqp_uri.as_str().connect_cancellable(|err| {
            eprintln!("Error encountered while attempting heartbeat. {}", err);
        })).from_err::<Error>();
        producer.join(consumer).and_then(|(prod, cons)| prod.0.create_channel()
            .from_err()
            .map(|chan| Self {
                consume_state: ConsumerState {
                    connection: cons.0,
                    heartbeat: Arc::new(cons.1),
                },
                prod_state: ProducerState {
                    connection: prod.0,
                    heartbeat: Arc::new(prod.1),
                    channel: chan,
                },
                group,
                subgroup,
            })
        ).from_err()
    }


    /// Publishes a payload for the provided event to the message brokers.
    /// You must serialize all payloads to a Vector of bytes.
    /// This method accepts an AMQPProperties struct which will set the AMQP properties for this message.
    /// See [here](https://docs.rs/amq-protocol/1.2.0/amq_protocol/protocol/basic/struct.AMQPProperties.html) for more details on the various AMQP properties.
    ///
    /// # Example
    /// -- snip --
    /// ```rust,norun
    /// AmqpBroker::new(AMQP_URI, "mygroup".to_string(), None)
    ///    .and_then(|broker| broker.publish(
    ///          "MESSAGE_CREATE",
    ///          b"{'content': 'Hi'}".to_vec(),
    ///          AmqpProperties::default().with_content_type("application/json")
    ///     ))
    /// ```
    ///
    pub fn publish(&self, evt: &str, payload: Vec<u8>, properties: AmqpProperties) -> impl Future<Item=Option<u64>, Error=Error> {
        debug!("Publishing event: {} to the AMQP server.", evt);
        self.prod_state.channel.basic_publish(
            self.group.as_ref(),
            evt,
            payload,
            BasicPublishOptions::default(),
            properties
        ).map_err(Error::from)
    }

    /// Attempts to consume the provided event. Returns a stream, which is populated with each incoming AMQP message.
    /// # Example
    /// ```rust,norun
    /// -- snip --
    /// AmqpBroker::new(addr, "mygroup", None)
    ///    .and_then(|broker| broker.consume("MESSAGE_CREATE"))
    ///    .for_each(|message| { // Poll the consumer stream.
    ///         println!("Message Event Received: {}", payload);
    ///
    ///         Ok(())
    ///     })
    ///     .map_err(|err| {
    ///         eprintln!("Failed to consume queue. {:?}", err);
    ///     })
    /// ```
    ///

    pub fn consume(&self, evt: &str) -> AmqpConsumer {
        let (tx, rx) = unbounded();
        let exch_opts = ExchangeDeclareOptions {
            durable: true,
            ..Default::default()
        };
        let queue_opts = QueueDeclareOptions {
            durable: true,
            ..Default::default()
        };
        let queue_name = match &self.subgroup {
            Some(g) => format!("{}:{}:{}", self.group, g, evt),
            None => format!("{}:{}", self.group, evt)
        };
        let group = self.group.clone();
        let event = evt.to_string();


        tokio::spawn(self.consume_state.connection.create_channel()
            .and_then({
                let group = group.clone();
                move |channel| channel.exchange_declare(
                    &group,
                    "direct",
                    exch_opts,
                    FieldTable::new(),
                ).map(|_| channel)
            })
            .and_then({
                let name = queue_name.clone();
                move |channel| channel.queue_bind(
                    &name,
                    &group,
                    &event,
                    QueueBindOptions::default(),
                    FieldTable::new(),
                ).map(|_| channel)
            })
            .and_then(move |channel| channel.queue_declare(
                &queue_name,
                queue_opts,
                FieldTable::new(),
            ).map(|queue| (channel, queue)))
            .and_then(|(channel, queue)| channel.basic_consume(
                &queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::new()
            ).map(|consumer| (channel, consumer)))
            .and_then(move |(channel, consumer)| {
                consumer.for_each(move |message| {
                    tx.unbounded_send(message.data).expect("Failed to send message to stream");
                    channel.basic_ack(message.delivery_tag, false)
                })
            })
            .map_err(|err| {
                error!("Failed to consume event: {:?}", err);
            }));

        AmqpConsumer::new(rx)
    }
}
