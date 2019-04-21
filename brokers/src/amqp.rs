use std::sync::Arc;
use std::time::Duration;

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

/// Central AMQP message brokers client. The preferred AMQP server is RabbitMQ, although the broker will work with AMQP-compliant server.
#[derive(Clone)]
pub struct AmqpBroker {
    /// The group used for consuming and producing messages.
    pub group: String,
    /// The subgroup used for consuming and producing messages.
    pub subgroup: Option<String>,
    pub_state: PubState,
    consume_state: ConsumerState,
}

impl AmqpBroker {
    /// Creates a new AMQP-based message broker, with the provided address, and groups.
    pub async fn new(uri: &str, group: String, subgroup: Option<String>) -> BrokerResult<AmqpBroker> {
        let strategy = Strategy::fibonacci(Duration::from_secs(2))
            .with_max_retries(10);
        let (publish, phb) = await!(strategy.retry(|| uri.connect_cancellable(|err| {
            eprintln!("Error encountered while attempting heartbeat. {}", err);
        })))?;
        let (consume, chb) = await!(strategy.retry(|| uri.connect_cancellable(|err| {
            eprintln!("Error encountered while attempting heartbeat. {}", err);
        })))?;
        let pub_channel = await!(publish.create_channel())?;

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
    ///
    /// use std::env::var;
    /// use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};
    ///
    /// fn main() {
    ///     tokio::run_async(async {
    ///         let addr = var("AMQP_URL").expect("No AMQP server address found");
    ///         let broker = await!(AmqpBroker::new(&addr, "MYGROUP".to_string(), None))
    ///             .expect("Failed to connect to broker");
    ///         let json = b"{'message': 'A MESSAGE HERE'}";
    /// 
    ///         match await!(broker.publish("MYQUEUE", json.to_vec(), properties)) {
    ///             Ok(_) => println!("{} Messages published.", publish_count),
    ///             Err(e) => eprintln!("An error was encountered during publish: {}", e)
    ///         }
    ///     }
    /// }
    /// ```
    ///
    ///
    ///

    pub async fn publish<'a>(&'a self, evt: &'a str, payload: Vec<u8>, properties: AmqpProperties) -> BrokerResult<()> {
        debug!("Publishing event: {} to the AMQP server.", evt);
        tokio::await!(self.pub_state.channel.basic_publish(
            &self.group,
            evt,
            payload,
            BasicPublishOptions::default(),
            properties
        ))?;

        Ok(())
    }

    /// Consumes all messages in the provided queue name.
    /// Returns [`AmqpConsumer`] stream of incoming messages.
    /// # Example
    /// ```rust,norun
    /// #![feature(futures_api, async_await, await_macro)]
    /// #[macro_use] extern crate tokio;
    ///
    /// use std::env::var;
    /// use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};
    ///
    /// fn main() {
    ///     tokio::run_async(async {
    ///         let addr = var("AMQP_URL").expect("No AMQP server address found");
    ///         let broker = await!(AmqpBroker::new(&addr, "MYGROUP".to_string(), None))
    ///             .expect("Failed to connect to broker");
    ///         let mut consumer = await!(broker.consume("YOURQUEUE"));
    ///
    ///         tokio::spawn_async(async move {
    ///             while let Some(Ok(message)) = await!(consumer.next()) {
    ///                 let string = std::str::from_utf8(&message);
    ///                 println!("Message received: {}", string);
    ///             }
    ///         });
    ///     }
    /// }
    /// ```
    ///
    /// [`AmqpConsumer`]: struct.AmqpConsumer.html
    ///
    pub async fn consume<'a>(&'a self, evt: &'a str) -> BrokerResult<AmqpConsumer> {
        let (tx, rx) = unbounded();
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
        let channel = await!(self.consume_state.connection.create_channel())?;
        await!(channel.exchange_declare(&self.group, "direct", exch_opts, FieldTable::new()))?;
        let queue = await!(channel.queue_declare(&queue_name, queue_opts, FieldTable::new()))?;
        debug!("Channel ID: {} has declared queue: {}", channel.id, queue_name);
        await!(channel.queue_bind(&queue_name, &self.group, evt, QueueBindOptions::default(), FieldTable::new()))?;
        let mut consumer = await!(channel.basic_consume(&queue, "", BasicConsumeOptions::default(), FieldTable::new()))?;

        tokio::spawn_async(async move {
            while let Some(Ok(mess)) = await!(consumer.next()) {
                tx.unbounded_send(mess.data).expect("Failed to send message to stream");
                await!(channel.basic_ack(mess.delivery_tag, false)).expect("Failed to acknowledge message");
            };
        });

        Ok(AmqpConsumer::new(rx))
    }
}
