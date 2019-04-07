use std::time::{Duration, Instant};

use futures::future::Future;
use tokio::prelude::*;
use tokio::timer::Interval;

use spectacles_model::snowflake::Snowflake;
use spectacles_rest::RestClient;

fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("No token provided");
    let client = RestClient::new(token, true);
    env_logger::init();

    tokio::run(Interval::new(Instant::now(), Duration::from_secs(1))
        .map_err(|_| ())
        .for_each(move |_| {
            client.channel(&Snowflake(536015945538338818)).create_message("Test")
                .map(|s| println!("Message sent to Discord. {:?}", s))
                .map_err(|err| panic!("Failed to send message to channel. {:#?}", err))
        })
    );
}