use std::env::var;

use futures::future::Future;

use spectacles_brokers::amqp::AmqpBroker;

// This example demonstrates a basic AMQP consumer.
// This example is meant to be ran with the producer example provided in this folder.
fn main() {
    let addr = var("AMQP_ADDR").expect("No AMQP server address found.");
    // We will begin by initializing our AMQP broker struct.
    // Here, we pass in our AMQP URI, and the group (exchange) that the broker will adhere to.
    // You may also specify a subgroup, if you would like to differentiate multiple queues for the same event on the same exchange.
    let connect = AmqpBroker::new(&addr, "test".to_string(), None);
    let result = connect.and_then(|broker| {
        // Now, we will subscribe and listen for the event we publish in the consumer.
        // We provide a callback function to the subscribe() method, which will be called when a message is received.
        broker.subscribe("HELLO".to_string(), |payload| {
            println!("Received Message: {}", payload);
        })
    }).map_err(|err| {
        eprintln!("An error was encountered during subscribe: {}", err);
    });

    // Here, we create our tokio runtime which allows us to run asynchronous code with ease.
    tokio::run(result);
}