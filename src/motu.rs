use crate::config::Config;
use crate::motu::channel::ChannelType;
use crate::motu::channel::Channel;
use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
use std::collections::HashMap;
use std::error::Error;
// use std::fmt::Display;


mod osc;
pub mod channel;

/// constant also needs the ip without port to be used in the http server
// const HTTP_PREFIX: &str = "/datastore";

// pub enum ChannelType {
//     Aux,
//     Chan,
//     Group,
// }

// impl Display for ChannelType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let channel_type = match self {
//             ChannelType::Aux => "aux",
//             ChannelType::Chan => "chan",
//             ChannelType::Group => "group",
//         };
//         write!(f, "{}", channel_type)
//     }
// }

// pub struct Channel {
//     number: i32,
//     channel_type: ChannelType,
// }
// impl Channel {
//     pub fn new(arg: i32, channel_type: ChannelType) -> Channel {
//         Channel {
//             number: arg,
//             channel_type,
//         }
//     }
//     pub fn channel_number(&self) -> i32 {
//         self.number
//     }
//     pub fn channel_type(&self) -> &ChannelType {
//         &self.channel_type()
//     }
// }

pub trait OscSender {
    fn new(address: &str, value: f32) -> Self;
}

impl OscSender for OscMessage {
    fn new(address: &str, value: f32) -> OscMessage {
        OscMessage {
            addr: address.to_string(),
            args: vec![OscType::Float(value)],
        }
    }
}

pub enum MotuCommand {
    EnableMonitoring,
    DisableMonitoring,
    PrintSettings,
    Volume { channel: Channel, volume: f32 },
    Send(Channel, Channel, f32),
    Mute(Channel),
    Unmute(Channel),
    Init,
}

pub struct Motu {
    client: osc::OscClient,
    aux_channels: HashMap<usize, String>,
    channels: HashMap<usize, String>,
    monitor_groups: HashMap<usize, String>,
}

impl Motu {
    pub fn new(ip_address: &str, config: &Config) -> Result<Motu, Box<dyn Error>> {
        let client = osc::OscClient::new(ip_address)?;
        Ok(Motu {
            client,
            aux_channels: config.aux_channels.clone(),
            channels: config.channels.clone(),
            monitor_groups: config.monitor_groups.clone(),
        })
    }

    pub fn run(&self, commands: Vec<MotuCommand>) -> Result<(), Box<dyn Error>> {
        for command in commands {
            // add a small pause of 40ms between commands?
            self.send(command)?;
        }
        Ok(())
    }

    pub fn test_osc(&self) {
        // todo!()
    }

    pub fn enable_monitoring(&self) -> Result<(), Box<dyn Error>> {
        for (group_index, group_name) in &self.monitor_groups {
            println!("Enabling monitoring for {} {}", group_index, group_name);
            let command =
                MotuCommand::Unmute(Channel::new(*group_index as i32, ChannelType::Group));
            self.send(command)?;
        }
        println!("Monitoring enabled");
        Ok(())
    }

    pub fn disable_monitoring(&self) -> Result<(), Box<dyn Error>> {
        for (group_index, group_name) in &self.monitor_groups {
            println!("Disabling monitoring for {} {}", group_index, group_name);
            let command = MotuCommand::Mute(Channel::new(*group_index as i32, ChannelType::Group));
            self.send(command)?;
        }
        println!("Monitoring disabled");
        Ok(())
    }

    pub fn print_settings(&self) -> Result<(), Box<dyn Error>> {
        let mut keys: Vec<_> = self.channels.keys().collect();
        keys.sort();
        println!("Channels:");
        for key in keys {
            println!("{}: {}", key, self.channels[key]);
        }
        let mut keys: Vec<_> = self.aux_channels.keys().collect();
        keys.sort();
        println!("Aux Channels:");
        for key in keys {
            println!("{}: {}", key, self.aux_channels[key]);
        }
        let mut keys: Vec<_> = self.monitor_groups.keys().collect();
        keys.sort();
        println!("Monitor Groups:");
        for key in keys {
            println!("{}: {}", key, self.monitor_groups[key]);
        }

        Ok(())
    }
    pub fn send(&self, command: MotuCommand) -> Result<(), Box<dyn Error>> {
        let message = match command {
            MotuCommand::EnableMonitoring => return self.enable_monitoring(),
            MotuCommand::DisableMonitoring => return self.disable_monitoring(),
            MotuCommand::PrintSettings => return self.print_settings(),
            MotuCommand::Volume { channel, volume } => {
                let (channel_number, channel_type) =
                    (channel.channel_number() as usize, channel.channel_type());
                if !self.channels.contains_key(&channel_number) {
                    return Err(format!("Channel {} is not defined", channel_number).into());
                }
                let address = format!("/mix/{}/{}/matrix/fader", channel_type, channel_number);

                OscMessage::new(&address, volume)
            }
            MotuCommand::Send(channel, aux_channel, value) => {
                let (channel_number, aux_channel_number, channel_type) = (
                    channel.channel_number() as usize,
                    aux_channel.channel_number() as usize,
                    channel.channel_type(),
                );
                if !self.channels.contains_key(&channel_number) {
                    return Err(format!("Channel {} is not defined", channel_number).into());
                }
                if !self.aux_channels.contains_key(&aux_channel_number) {
                    return Err(format!("Aux channel {} is not defined", aux_channel_number).into());
                }
                let address = format!(
                    "/mix/{}/{}/matrix/aux/{}/send",
                    channel_type, channel_number, aux_channel_number
                );

                OscMessage::new(&address, value)
            }
            MotuCommand::Mute(channel) => {
                let channel_number = channel.channel_number();
                let address = format!("/mix/group/{}/matrix/mute", channel_number);
                OscMessage::new(&address, 1.0)
            }
            MotuCommand::Unmute(channel) => {
                let channel_number = channel.channel_number();
                let address = format!("/mix/group/{}/matrix/mute", channel_number);
                OscMessage::new(&address, 0.0)
            }
            MotuCommand::Init => return self.init_settings(),
        };

        let packet = OscPacket::Message(message);
        self.client.send(packet)?;
        Ok(())
    }

    fn init_settings(&self) -> Result<(), Box<dyn Error>> {
        // loop through all channels and set the volume to 1.0, and then loop through all sends for each channel and set them to 0
        // in a second loop after this, loop through all the monitor groups and unmute them
        let mut commands = vec![];
        let delay: u64 = 2;
        for (channel_index, channel_name) in &self.channels {
            println!(
                "Setting volume for channel {} {}",
                channel_index, channel_name
            );
            let command = MotuCommand::Volume {
                channel: Channel::new(*channel_index as i32, ChannelType::Chan),
                volume: 1.0,
            };
            std::thread::sleep(std::time::Duration::from_millis(delay));
            commands.push(command);
            for (aux_channel_index, aux_channel_name) in &self.aux_channels {
                println!(
                    "Setting send for channel {} {} to aux channel {} {}",
                    channel_index, channel_name, aux_channel_index, aux_channel_name
                );
                let command = MotuCommand::Send(
                    Channel::new(*channel_index as i32, ChannelType::Chan),
                    Channel::new(*aux_channel_index as i32, ChannelType::Aux),
                    0.0,
                );
                std::thread::sleep(std::time::Duration::from_millis(delay));
                commands.push(command);
            }
        }

        for (group_index, group_name) in &self.monitor_groups {
            println!("Unmuting monitor group {} {}", group_index, group_name);
            let command =
                MotuCommand::Unmute(Channel::new(*group_index as i32, ChannelType::Group));
            std::thread::sleep(std::time::Duration::from_millis(delay));
            commands.push(command);
        }

        self.run(commands)?;

        Ok(())
    }
}
