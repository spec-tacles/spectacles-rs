#[macro_use] extern crate log;

use clap::{App, AppSettings, Arg, SubCommand};
use log::Level::Info;

mod sharder;
mod errors;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main () {
    if !log_enabled!(Info) {
        std::env::set_var("RUST_LOG", "INFO");
    };
    let _ = kankyo::load();
    env_logger::init();
    let results = App::new("Spectacles")
        .setting(AppSettings::SubcommandRequired)
        .version(VERSION)
        .about(ABOUT)
        .subcommand(SubCommand::with_name("shard")
            .about("Spawn Discord shards and publish events to a message broker.")
            .arg(Arg::with_name("count")
                .short("c")
                .long("count")
                .help("The amount of shards to spawn. If omitted, the recommended amount of shards will be spawned.")
                .value_name("COUNT")
            )
            .arg(Arg::with_name("url")
                .short("u")
                .long("amqpurl")
                .help("The AMQP server to publish events to.")
                .value_name("URL")
            )
            .arg(Arg::with_name("group")
                .short("g")
                .long("group")
                .help("The AMQP group (exchange) that will be used to register queues for Discord Events.")
                .value_name("GROUP")
            )
            .arg(Arg::with_name("subgroup")
                .short("sg")
                .long("subgroup")
                .help("The AMQP subgroup (exchange) that will be used to register queues for Discord Events.")
                .value_name("SUBGROUP")
            )
            .arg(Arg::with_name("token")
                .short("t")
                .long("token")
                .help("The Discord token that will be used to connect to the gateway.")
                .value_name("TOKEN")
            )
        ).get_matches();

    match results.subcommand() {
        ("shard", Some(matches)) => {
            sharder::parse_args(matches).map_err(|err| {
                error!("Failed at spawning shards. {}", err);
            }).unwrap();
        },
        _ => {}
    }
}
