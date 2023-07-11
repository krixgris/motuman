use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::motu::MotuCommand;

use crate::args::IpEndpoint;

#[derive(Debug, Deserialize)]
pub enum MidiCommand {
    #[serde(rename = "vol")]
    Vol(usize),
    // #[serde(rename = "send")]
    send(usize, usize),
    Init,
    MonitorOn,
    MonitorOff
}

#[derive(Debug, Deserialize)]
pub struct MidiConfig {
    input: String,
    output: String,
    pub midi_channel: u8,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub ip_address: IpEndpoint,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Config {
    // #[serde_as(as = "DisplayFromStr")]
    #[serde(skip)]
    pub ip_address: IpEndpoint,
    pub network: NetworkConfig,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub aux_channels: HashMap<usize, String>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub channels: HashMap<usize, String>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub monitor_groups: HashMap<usize, String>,
    pub midi_config: Option<MidiConfig>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub midi_mapping_cc: HashMap<usize, MidiCommand>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub midi_mapping_note_on: HashMap<usize, String>,
}

impl Config {
    pub fn build(file_name: String, arg_ip: Option<IpEndpoint>) -> Result<Config, Box<dyn Error>> {
        let config_file = fs::read_to_string(dbg!(file_name))?;

        let mut config: Config = toml::from_str(&config_file)?;

        if let Some(ip) = arg_ip {
            config.ip_address = ip;
        } else {
            config.ip_address = config.network.ip_address;
        }

        Ok(config)
    }
}
