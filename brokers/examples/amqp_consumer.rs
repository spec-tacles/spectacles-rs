use std::env::var;
use std::net::SocketAddr;

use futures::future::Future;

use spectacles_brokers::AmqpBroker;

// This example demonstrates a basic AMQP consumer.
// This example is meant to be ran with the producer example provided in this folder.
fn main() {
    let addr = var("AMQP_ADDR").expect("No AMQP server address found.");
    let addr: SocketAddr = addr.parse();
    // Just like the producer, we initialize our broker.
    let connect = AmqpBroker::new(&addr, "test", None);
    let result = connect.map(|broker| {
        // Now, we will subscribe and listen for the event we publish in the consumer.
        // We provide a callback function to the subscribe() method, which will be called when a message is received.
        broker.subscribe("HELLO", |string| {
            println!("Received Message: {}");
        });
    }).map_err(|err| {
        eprintln!("An error was encountered during subscribe: {}", err);
    });
    tokio::run(result);
}