use std::env::var;

use futures::future::Future;

use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};

// This example demonstrates a basic AMQP producer.
// This example is meant to be ran with the consumer example provided in this folder.
fn main() {
    let addr = var("AMQP_URL").expect("No AMQP server address found.");
    // Just like the consumer, we initialize our producer.
    let connect = AmqpBroker::new(addr, "test".to_string(), None)
        .map_err(|err| {
            eprintln!("Failed to initialize broker. {:?}", err);
        });
    let producer = connect.and_then(|broker| {
        // Here, we will publish an event with a name of HELLO to the message broker, and a basic content type for our AMQP properties.
        // We create a mock JSON string to send to replicate a real-world JSON payload.
        let json = b"{'message': 'Example Publish.'}";
        let props = AmqpProperties::default().with_content_type("application/json".to_string());
        broker.publish("HELLO", json.to_vec(), props).map_err(|err| {
            eprintln!("An error was encountered during publish: {}", err);
        })
    }).map(|_| {
        println!("Message publish succeeded, check the other window!");
        std::process::exit(0);
    });

    tokio::run(producer);
}