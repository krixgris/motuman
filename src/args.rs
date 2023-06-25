use crate::motu;
use clap::Parser;
// use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct IpAddress {
    octets: [u8; 4],
}
impl std::str::FromStr for IpAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octets: Vec<&str> = s.split('.').collect();

        if octets.len() != 4 {
            return Err(String::from("Invalid IP address format"));
        }

        let mut ip = IpAddress { octets: [0; 4] };

        for (i, octet) in octets.iter().enumerate() {
            match octet.parse::<u8>() {
                Ok(value) => ip.octets[i] = value,
                Err(_) => return Err(String::from("Invalid IP address format")),
            }
        }

        Ok(ip)
    }
}

impl std::convert::From<&str> for IpAddress {
    fn from(ip: &str) -> Self {
        let octets: Vec<&str> = ip.split('.').collect();
        if octets.len() != 4 {
            return IpAddress { octets: [0; 4] };
        }

        let mut ip = IpAddress { octets: [0; 4] };

        for (i, octet) in octets.iter().enumerate() {
            match octet.parse::<u8>() {
                Ok(value) => ip.octets[i] = value,
                Err(_) => return IpAddress { octets: [0; 4] },
            }
        }
        ip
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.octets[0], self.octets[1], self.octets[2], self.octets[3])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IpEndpoint {
    address: IpAddress,
    port: u16,
}

impl fmt::Display for IpEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.address, self.port)
    }
}

impl IpEndpoint {
    pub fn new(address:String) -> Self {
        address.parse::<IpEndpoint>().unwrap()
    }
}

impl std::str::FromStr for IpEndpoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() > 2 {
            return Err(String::from("Invalid IP endpoint format"));
        }

        let address = match parts[0].parse::<IpAddress>() {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        let port = match parts.get(1) {
            Some(port_str) => match port_str.parse::<u16>() {
                Ok(value) => value,
                Err(_) => return Err(String::from("Invalid port number format")),
            },
            None => 8000,
        };

        Ok(IpEndpoint { address, port })
    }
}

impl std::convert::From<&str> for IpEndpoint {
    fn from(ip: &str) -> Self {
        let parts: Vec<&str> = ip.split(':').collect();

        if parts.len() > 2 {
            return IpEndpoint { address: IpAddress { octets: [0; 4] }, port: 8000 };
        }

        let address = match parts[0].parse::<IpAddress>() {
            Ok(value) => value,
            Err(e) => return IpEndpoint { address: IpAddress { octets: [0; 4] }, port: 8000 },
        };

        let port = match parts.get(1) {
            Some(port_str) => match port_str.parse::<u16>() {
                Ok(value) => value,
                Err(_) => return IpEndpoint { address: IpAddress { octets: [0; 4] }, port: 8000 },
            },
            None => 8000,
        };

        IpEndpoint { address, port }
    }
}

// impl Into<String> for IpEndpoint {
//     fn into(self) -> String {
//         "{}:{}".format(self.address, self.port)
//     }
// }


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
    pub ip_address: Option<IpEndpoint>,
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

    // pub fn ip_address(&self) -> Option<String> {
    //     self.ip_address.as_ref().map(|ip| {
    //         if ip.contains(':') {
    //             ip.clone()
    //         } else {
    //             format!("{}:{}", ip, self.port.unwrap_or(8000))
    //         }
    //     })
    // }

    // pub fn ip_address_only(&self) -> Option<String> {
    //     // validate that the IP address is valid
    //     // todo!("validate that the IP address is valid");
    //     // split my ip on : and into 4 u8 parts that i can use in the Ipv4Addr::new()
    //     if let Some(ip) = &self.ip_address {
    //         if ip.contains(':') {
    //             let parts: Vec<&str> = ip.split(':').collect();
    //             let octets: Vec<u8> = parts.iter().map(|part| part.parse().unwrap()).collect();
    //             let ipv4_addr = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);
    //             return Some(ipv4_addr.to_string());
    //         } else {
    //             return Some(ip.clone());
    //         }
    //     }
    //     None
    // }

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
