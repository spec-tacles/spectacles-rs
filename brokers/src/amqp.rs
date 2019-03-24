use std::sync::Arc;

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
    client::{Client as LapinClient, HeartbeatHandle},
    types::FieldTable,
};
use lapin_futures_native_tls::lapin::channel::BasicProperties;
use tokio::prelude::*;

use crate::errors::BrokerResult;

/// A shortcut for the AMQP basic properties.
pub type AmqpProperties = BasicProperties;

#[derive(Clone)]
struct PubState {
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
    /// A dedicated connection for publishing AMQP messages.
    pub pub_state: PubState,
    /// A dedicated connection for consuming AMQP messages.
    pub consume_state: ConsumerState,
    /// The group used for consuming and producing messages.
    pub group: String,
    /// The subgroup used for consuming and producing messages.
    pub subgroup: Option<String>
}

impl AmqpBroker {
    /// Creates a new AMQP-based message broker, with the provided address, and groups.
    pub async fn new(uri: &str, group: String, subgroup: Option<String>) -> BrokerResult<AmqpBroker> {
        let (publish, phb) = tokio::await!(uri.connect_cancellable(|err| {
            eprintln!("Error encountered while attempting heartbeat. {}", err);
        }))?;
        let pub_channel = tokio::await!(publish.create_channel())?;
        let (consume, chb) = tokio::await!(uri.connect_cancellable(|err| {
            eprintln!("Error encountered while attempting heartbeat. {}", err);
        }))?;

        Ok(Self {
            consume_state: ConsumerState {
                connection: consume,
                heartbeat: Arc::new(chb),
            },
            pub_state: PubState {
                connection: publish,
                heartbeat: Arc::new(phb),
                channel: pub_channel,
            },
            group,
            subgroup,
        })
    }

    /// Publishes a payload for the provided event to the message brokers.
    /// You must serialize all payloads to a Vector of bytes.
    /// This method accepts an AMQPProperties struct which will set the AMQP properties for this message.
    /// See [here](https://docs.rs/amq-protocol/1.2.0/amq_protocol/protocol/basic/struct.AMQPProperties.html) for more details on the various AMQP properties.
    ///
    /// # Example
    /// ```rust,norun
    /// #![feature(futures_api, async_await, await_macro)]
    /// #[macro_use] extern crate tokio;
    /// use std::env::var;
    /// use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};
    ///
    /// fn main() {
    ///   tokio::run_async(async {
    ///     let addr = var("AMQP_URL").expect("No AMQP server address found");
    ///     let broker = await!(AmqpBroker::new(&addr, "MYGROUP".to_string(), None))
    ///         .expect("Failed to connect to broker");
    ///     let json = b"{'message': 'A MESSAGE HERE'}";
    ///     match await!(broker.publish("MYQUEUE", json.to_vec(), properties)) {
    ///         Ok(_) => println!("{} Messages published.", publish_count),
    ///         Err(e) => eprintln!("An error was encountered during publish: {}", e)
    ///      }
    ///   }
    /// }
    /// ```
    ///
    ///
    ///

    pub async fn publish<'a>(&'a self, evt: &'a str, payload: Vec<u8>, properties: AmqpProperties) -> BrokerResult<()> {
        debug!("Publishing event: {} to the AMQP server.", evt);
        tokio::await!(self.pub_state.channel.basic_publish(
            self.group.as_ref(),
            evt,
            payload,
            BasicPublishOptions::default(),
            properties
        ))?;

        Ok(())
    }

    /// Subscribes to the provided event, with a callback that is called when an event is received.
    /// # Example
    /// ```rust,norun
    /// #![feature(futures_api, async_await, await_macro)]
    /// #[macro_use] extern crate tokio;
    /// use std::env::var;
    /// use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};
    ///
    /// fn main() {
    ///   tokio::run_async(async {
    ///     let addr = var("AMQP_URL").expect("No AMQP server address found");
    ///     let broker = await!(AmqpBroker::new(&addr, "MYGROUP".to_string(), None))
    ///         .expect("Failed to connect to broker");
    ///     let json = b"{'message': 'Example Publish.'}";
    ///     await!(broker.subscribe("MYQUEUE".to_string(), |payload| {
    ///          println!("Message received: {}", payload);
    ///      })).expect("Failed to subscribe to this event.");
    ///   }
    /// }
    /// ```
    ///
    pub async fn subscribe<C>(self, evt: String, mut cb: C) -> BrokerResult<AmqpBroker>
        where C: FnMut(&str) + Send + Sync + 'static
    {
        let queue_name = match &self.subgroup {
            Some(g) => format!("{}:{}:{}", self.group, g, evt),
            None => format!("{}:{}", self.group, evt)
        };
        let exch_opts = ExchangeDeclareOptions {
            durable: true,
            ..Default::default()
        };
        let queue_opts = QueueDeclareOptions {
            durable: true,
            ..Default::default()
        };
        let state = self.consume_state.clone();
        let group = self.group.clone();
        tokio::spawn_async(async move {
            let channel = tokio::await!(state.connection.create_channel())
                .expect("Failed to create channel");
            tokio::await!(channel.exchange_declare(&group, "direct", exch_opts, FieldTable::new()))
                .expect("Failed to declare exchange");
            let queue = tokio::await!(channel.queue_declare(&queue_name, queue_opts, FieldTable::new()))
                .expect("Failed to declare queue");
            debug!("Channel ID: {} has declared queue: {}", channel.id, queue_name);
            tokio::await!(channel.queue_bind(&queue_name, &group, &evt, QueueBindOptions::default(), FieldTable::new()))
                .expect("Failed to bind channel to queue");
            let mut consumer = tokio::await!(channel.basic_consume(&queue, "", BasicConsumeOptions::default(), FieldTable::new()))
                .expect("Failed to create consumer");

            while let Some(Ok(mess)) = tokio::await!(consumer.next()) {
                if let Err(e) = tokio::await!(channel.basic_ack(mess.delivery_tag, false)) {
                    error!("Failed to acknowledge message: {}", e);
                };
                let payload = std::str::from_utf8(&mess.data).expect("Invalid UTF8 payload sent");
                cb(payload);
            };
        });

        Ok(self)
    }
}
