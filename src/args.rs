use crate::motu;
use clap::Parser;
// use std::net::IpAddr;
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "./motu_config.toml")]
    pub config: String,
    #[arg(short, long)]
    pub monitor: Option<bool>,
    #[arg(short, long)]
    pub channel: Option<i32>,
    #[arg(short, long)]
    pub volume: Option<f32>,
    #[arg(long = "ip")]
    pub ip_address: Option<String>,
    #[arg(long = "port", default_value = "8000")]
    pub port: Option<u16>,
    #[arg(short, long)]
    pub list_channels: bool,
    #[arg(short, long)]
    pub aux_channel: Option<i32>,
    #[arg(short, long)]
    pub send_amount: Option<f32>,
    #[arg(short, long)]
    pub init: bool,
}

impl Args {
    pub fn init() -> Self {
        Self::parse()
    }

    pub fn ip_address(&self) -> Option<String> {
        self.ip_address.as_ref().map(|ip| {
            if ip.contains(':') {
                ip.clone()
            } else {
                format!("{}:{}", ip, self.port.unwrap_or(8000))
            }
        })
    }

    pub fn ip_address_only(&self) -> Option<String> {
        // validate that the IP address is valid
        // todo!("validate that the IP address is valid");
        // split my ip on : and into 4 u8 parts that i can use in the Ipv4Addr::new()
        if let Some(ip) = &self.ip_address {
            if ip.contains(':') {
                let parts: Vec<&str> = ip.split(':').collect();
                let octets: Vec<u8> = parts.iter().map(|part| part.parse().unwrap()).collect();
                let ipv4_addr = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);
                return Some(ipv4_addr.to_string());
            } else {
                return Some(ip.clone());
            }
        }
        None
    }

    pub fn config_file_name(&self) -> String {
        self.config.clone()
    }

    pub fn motu_commands(&self) -> Vec<motu::MotuCommand> {
        let mut motu_commands: Vec<motu::MotuCommand> = vec![];

        let init = self.init;
        if init {
            motu_commands.push(motu::MotuCommand::Init);
        }

        if let Some(true) = self.monitor {
            motu_commands.push(motu::MotuCommand::EnableMonitoring);
        } else if let Some(false) = self.monitor {
            motu_commands.push(motu::MotuCommand::DisableMonitoring);
        }
        let channel = self.channel;
        let send_to_channel = self.aux_channel;
        let send_amount = self.send_amount;
        let volume = self.volume;

        if let (Some(channel), Some(send_to_channel)) = (channel, send_to_channel) {
            motu_commands.push(motu::MotuCommand::Send {
                channel: motu::channel::Channel::new(channel, motu::channel::ChannelType::Chan),
                aux_channel: motu::channel::Channel::new(
                    send_to_channel,
                    motu::channel::ChannelType::Aux,
                ),
                value: send_amount.unwrap_or(0.0),
            })
        }

        if let (Some(channel), Some(volume)) = (channel, volume) {
            motu_commands.push(motu::MotuCommand::Volume {
                channel: motu::channel::Channel::new(channel, motu::channel::ChannelType::Chan),
                volume,
            })
        }

        let list_channels = self.list_channels;
        if list_channels {
            motu_commands.push(motu::MotuCommand::PrintSettings);
        }

        motu_commands
    }
}
