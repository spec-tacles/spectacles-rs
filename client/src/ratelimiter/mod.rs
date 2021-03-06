use std::env;
use std::fs;
use std::net::SocketAddr;

use clap::ArgMatches;

use crate::errors::*;

use self::server::RatelimitServer;

pub mod server;
pub mod bucket;

#[derive(Deserialize, Serialize, Clone)]
pub struct RatelimitOptions {
    pub url: String,
    pub address: SocketAddr,
    pub config_path: Option<String>,
}

pub fn bootstrap(results: &ArgMatches) -> Result<()> {
    let options = if results.value_of("config_path").is_some() || env::var("CONFIG_FILE_PATH").is_ok() {
        let path = results.value_of("CONFIG_PATH")
            .map(|s| s.to_string())
            .unwrap_or(env::var("CONFIG_FILE_PATH").unwrap());
        parse_config_file(path.to_string())?
    } else {
        parse_argv(results)?
    };

    let server = RatelimitServer::new(options);
    server.start();

    Ok(())
}

fn parse_config_file(path: String) -> Result<RatelimitOptions> {
    let file = fs::read_to_string(path)?;

    if file.ends_with(".json") {
        Ok(serde_json::from_str::<RatelimitOptions>(&file)?)
    } else if file.ends_with(".toml") {
        Ok(toml::from_str::<RatelimitOptions>(&file)?)
    } else {
        Err(Error::InvalidFile)
    }
}

pub fn parse_argv(args: &ArgMatches) -> Result<RatelimitOptions> {
    let address = args.value_of("server_address")
        .map(String::from)
        .unwrap_or(env::var("SERVER_ADDR").expect("No server was provided in a config file, command line argument or environment variable."));
    let address = address.parse::<SocketAddr>()?;

    Ok(RatelimitOptions {
        address,
        url: String::from("https://discordapp.com"),
        config_path: None,
    })
}