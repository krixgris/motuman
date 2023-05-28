use std::env;
use std::error::Error;
use std::fs;
use toml::Value;

pub struct Config {
    pub ip_address: String,
    pub monitor: bool,
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
        let config_file = fs::read_to_string("./config.toml")?;
        let config: Value = toml::from_str(&config_file)?;

        let network = config
            .get("network")
            .and_then(|v| v.as_table())
            .ok_or("Missing [network] table")?;
        let ip_address = network
            .get("ip_address")
            .and_then(|v| v.as_str())
            .ok_or("Missing ip_address field")?
            .to_string();

        let mut monitor = false;
        while let Some(arg) = args.next() {
            if arg == "--monitor=on" {
                monitor = true;
            }
            if arg == "--monitor=off" {
                monitor = false;
            }
        }

        Ok(Config {
            ip_address,
            monitor,
        })
    }
}