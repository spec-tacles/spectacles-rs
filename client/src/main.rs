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

    match argv::get_args().subcommand() {
        ("shard", Some(matches)) => {
            sharder::parse_args(matches).unwrap_or_else(|err| {
                error!("Failed at spawning shards. {:?}", err);
            });
        },
        ("ratelimit", Some(matches)) => {
            ratelimiter::bootstrap(matches).unwrap_or_else(|err| {
                error!("Failed to bootstrap rate limiter service. {:?}", err);
            });
        },
        _ => {}
    }
}
