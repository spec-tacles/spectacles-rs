#![feature(futures_api, async_await, await_macro)]
#[macro_use]
extern crate tokio;

use std::env::var;

use spectacles_brokers::amqp::{AmqpBroker, AmqpProperties};

// This example demonstrates a basic AMQP producer.
// This example is meant to be ran with the consumer example provided in this folder.
fn main() {
    // Here, we run our async function on the Tokio executor.
    // tokio::run_async(try_main());
    tokio::run_async(async {
        let addr = var("AMQP_URL").expect("No AMQP server address found");
        let publish_count = var("COUNT").expect("No count detected").parse::<i32>().expect("invalid integer");
        // Just like the consumer, we initialize our producer.
        let broker = await!(AmqpBroker::new(&addr, "test".to_string(), None))
            .expect("Failed to connect to broker");
        // Here, we will publish an event with a name of HELLO to the message broker, and a basic content type for our AMQP properties.
        // We create a mock JSON string to send to replicate a real-world JSON payload.
        let json = b"{'message': 'Example Publish.'}";

        for num in 0..publish_count {
            let properties = AmqpProperties::default().with_content_type("application/json".to_string());
            if let Err(e) = await!(broker.publish("HELLO", json.to_vec(), properties)) {
                eprintln!("An error was encountered during publish: {}", e)
            }
        };

        println!("{} Messages published.", publish_count);
    })
}
