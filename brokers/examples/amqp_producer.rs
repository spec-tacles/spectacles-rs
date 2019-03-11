use std::env::var;
use std::net::SocketAddr;

use futures::future::Future;

use spectacles_brokers::AmqpBroker;

// This example demonstrates a basic AMQP producer.
// This example is meant to be ran with the consumer example provided in this folder.
fn main() {
    let addr = var("AMQP_ADDR").expect("No AMQP server address found.");
    let addr: SocketAddr = addr.parse().expect("Malformed URL provided, please try another URL.");
    // We will begin by initializing our AMQP broker struct.
    // Here, we pass in our socket address, the group (exchange) that the broker will adhere to.
    // You may also specify a subgroup, if you would like to differentiate multiple queues for the same event on the same exchange.
    let connect = AmqpBroker::new(&addr, "test".to_string(), None);
    let result = connect.and_then(|broker| {
        // Here, we will publish an event with a name of HELLO to the message broker.
        // We create a mock JSON string to send to replicate a real-world JSON payload.
        let json = r#"{"message": "Example Publish."}"#.as_bytes();
        broker.publish("HELLO", json.to_vec())
    });
    let success = result.map(|_| {
        println!("Message publish succeeded, check the other window!");
    }).map_err(|err| {
        eprintln!("An error was encountered during publish: {}", err);
    });
    // Here, we create our tokio runtime which allows us to run asynchronous code with ease.
    tokio::run(success);
}