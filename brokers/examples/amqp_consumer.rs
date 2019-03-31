#![feature(futures_api, async_await, await_macro)]
#[macro_use]
extern crate tokio;

use std::env::var;

use spectacles_brokers::amqp::AmqpBroker;

// This example demonstrates a basic AMQP consumer.
// This example is meant to be ran with the producer example provided in this folder.
fn main() {
    tokio::run_async(async {
        let addr = var("AMQP_URL").expect("No AMQP server address found.");
        // We will begin by initializing our AMQP broker struct.
        // Here, we pass in our AMQP URI, and the group (exchange) that the broker will adhere to.
        // You may also specify a subgroup, if you would like to differentiate multiple queues for the same event on the same exchange.
        let broker = await!(AmqpBroker::new(&addr, "test".to_string(), None))
            .expect("Failed to connect to broker");
        println!("I'm now listening for messages!");
        // Here we attach a callback to the subscribe() method that will be called when we receive a payload for our event name.
        broker.subscribe("HELLO", recv_payload);
    });
}


async fn recv_payload(payload: String) {
    println!("Received payload: {}", payload);
}