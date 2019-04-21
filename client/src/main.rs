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
    pretty_env_logger::init_timed();

    match argv::get_args().subcommand() {
        ("shard", Some(matches)) => {
            sharder::parse_args(matches).expect("Failed to bootstrap sharder");
        },
        ("ratelimit", Some(matches)) => {
            ratelimiter::bootstrap(matches).expect("Failed to bootstrap rate limiter service");
        },
        _ => {}
    }
}
