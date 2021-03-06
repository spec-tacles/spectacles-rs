use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

pub fn get_args() -> ArgMatches<'static> {
    App::new("Spectacles")
        .setting(AppSettings::SubcommandRequired)
        .version(VERSION)
        .about(ABOUT)
        .subcommand(SubCommand::with_name("shard")
            .about("Spawn Discord shards and publish events to a message broker.")
            .arg(Arg::with_name("config_path")
                .long("config-path")
                .help("The location of the configuration file that you would like to use. Supports TOML and JSON.")
                .value_name("PATH")
            )
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
        )
        .subcommand(SubCommand::with_name("ratelimit")
            .about("Starts an HTTP ratelimiter proxy, which can ratelimit your HTTP clients.")
            .arg(Arg::with_name("server_address")
                .short("a")
                .long("address")
                .help("The TCP address on which to listen for requests.")
                .value_name("ADDRESS")
            )
            .arg(Arg::with_name("config_path")
                .short("c")
                .long("config-path")
                .value_name("PATH")
                .help("The location of the configuration file that you would like to use. Supports TOML and JSON.")
            )
        ).get_matches()
}