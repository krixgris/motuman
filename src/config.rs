use std::env;
use std::error::Error;
use std::fs;
use toml::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub ip_address: String,
    pub monitor: bool,
    pub aux_channels: HashMap<String, usize>,
    pub channels: HashMap<String, usize>,
    pub monitor_groups: HashMap<String, usize>,
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

        let aux_channels = config
            .get("aux_channels")
            .and_then(|v| v.as_table())
            .map(|table| {
                table
                    .iter()
                    .filter_map(|(k, v)| {
                        v.as_integer().map(|i| (k.clone(), i as usize))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let channels = config
            .get("channels")
            .and_then(|v| v.as_table())
            .map(|table| {
                table
                    .iter()
                    .filter_map(|(k, v)| {
                        v.as_integer().map(|i| (k.clone(), i as usize))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let monitor_groups = config
            .get("monitor_groups")
            .and_then(|v| v.as_table())
            .map(|table| {
                table
                    .iter()
                    .filter_map(|(k, v)| {
                        v.as_integer().map(|i| (k.clone(), i as usize))
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(Config {
            ip_address,
            monitor,
            aux_channels,
            channels,
            monitor_groups,
        })
    }
}


// pub struct Config {
//     pub ip_address: String,
//     pub monitor: bool,
// }

// impl Config {
//     pub fn build(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
//         let config_file = fs::read_to_string("./config.toml")?;
//         let config: Value = toml::from_str(&config_file)?;

//         let network = config
//             .get("network")
//             .and_then(|v| v.as_table())
//             .ok_or("Missing [network] table")?;
//         let ip_address = network
//             .get("ip_address")
//             .and_then(|v| v.as_str())
//             .ok_or("Missing ip_address field")?
//             .to_string();

//         let mut monitor = false;
//         while let Some(arg) = args.next() {
//             if arg == "--monitor=on" {
//                 monitor = true;
//             }
//             if arg == "--monitor=off" {
//                 monitor = false;
//             }
//         }

//         Ok(Config {
//             ip_address,
//             monitor,
//         })
//     }
// }
