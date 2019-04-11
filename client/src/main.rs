#![feature(futures_api, async_await, await_macro)]
#![recursion_limit = "128"]
#[macro_use]
extern crate clap;
#[warn(rust_2018_idioms)]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;

use log::Level::Debug;

mod sharder;
mod ratelimiter;
mod errors;
mod argv;


fn main () {
    if !log_enabled!(Debug) {
        std::env::set_var("RUST_LOG", "INFO");
    };
    let _ = kankyo::load();
    env_logger::init();

    let args = argv::get_args();
    match args.subcommand() {
        ("shard", Some(matches)) => {
            let mts = matches.clone();
            tokio::run_async(sharder::parse_args(mts));
        },
        ("ratelimit", Some(matches)) => {
            ratelimiter::bootstrap(matches).expect("Failed to begin ratelimiter service");
        },
        _ => {}
    };
}
