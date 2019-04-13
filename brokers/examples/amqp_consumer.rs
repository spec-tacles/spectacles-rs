use std::env::var;

use tokio::prelude::*;

use spectacles_brokers::amqp::AmqpBroker;

// This example demonstrates a basic AMQP consumer.
// This example is meant to be ran with the producer example provided in this folder.
fn main() {
    let addr = var("AMQP_ADDR").expect("No AMQP server address found.");
    // We will begin by initializing our AMQP broker struct.
    // Here, we pass in our AMQP URI, and the group (exchange) that the broker will adhere to.
    // You may also specify a subgroup, if you would like to differentiate multiple queues for the same
    // event on the same exchange.
    let connect = AmqpBroker::new(addr, "test".to_string(), None)
        .map_err(|err| {
            eprintln!("Failed to create AMQP broker: {:?}", err);
        });
    let result = connect.and_then(|broker| {
        println!("Broker created, listening for messages.");
        // Now, we will subscribe and listen for the event we publish in the consumer.
        // The consume() method returns a stream of AMQP methods represented as a buffer of the message contents.
        broker.consume("HELLO").for_each(|payload| {
            let text = std::str::from_utf8(&payload).expect("Failed to deserialize payload");
            println!("Received Payload: {:?}", text);

            Ok(())
        })
    });

    // Here, we create our tokio runtime which allows us to run asynchronous code with ease.
    tokio::run(result);
}