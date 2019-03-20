#[macro_use] extern crate log;

use log::Level::Info;

mod sharder;
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
            sharder::parse_args(matches).map_err(|err| {
                error!("Failed at spawning shards. {}", err);
            }).unwrap();
        },
        _ => {}
    }
}
