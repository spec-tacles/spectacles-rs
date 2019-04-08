#![feature(futures_api, async_await, await_macro)]
#![recursion_limit = "128"]
#[macro_use]
extern crate clap;
#[warn(rust_2018_idioms)]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;

use log::Level::Info;

mod sharder;
mod ratelimiter;
mod errors;
mod argv;


fn main () {
    if !log_enabled!(Info) {
        std::env::set_var("RUST_LOG", "INFO");
    };
    let _ = kankyo::load();
    env_logger::init();

    tokio::run_async(async {
        match argv::get_args().subcommand() {
            ("shard", Some(matches)) => {
                await!(sharder::parse_args(matches.clone())).expect("Failed to spawn shards")
            },
            ("ratelimit", Some(matches)) => {
                ratelimiter::bootstrap(matches).expect("Failed to begin ratelimiter service");
            },
            _ => {}
        };
    });
}
