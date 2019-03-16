use std::env::var;
use std::net::SocketAddr;

use futures::future::Future;

use spectacles_brokers::AmqpBroker;

// This example demonstrates a basic AMQP consumer.
// This example is meant to be ran with the producer example provided in this folder.
fn main() {
    let addr = var("AMQP_ADDR").expect("No AMQP server address found.");
    let addr: SocketAddr = addr.parse().expect("Malformed URL provided, please try another URL.");
    // We will begin by initializing our AMQP broker struct.
    // Here, we pass in our socket address, the group (exchange) that the broker will adhere to.
    // You may also specify a subgroup, if you would like to differentiate multiple queues for the same event on the same exchange.
    let connect = AmqpBroker::new(&addr, "test".to_string(), None);
    let result = connect.map(|broker| {
        // Now, we will subscribe and listen for the event we publish in the consumer.
        // We provide a callback function to the subscribe() method, which will be called when a message is received.
        broker.subscribe("HELLO", |payload| {
            println!("Received Message: {}", payload);
        });
    }).map_err(|err| {
        eprintln!("An error was encountered during subscribe: {}", err);
    });

    // Here, we create our tokio runtime which allows us to run asynchronous code with ease.
    tokio::run(result);
}