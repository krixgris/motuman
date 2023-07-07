use crate::config::Config;
use crate::motu::channel::Channel;
use crate::motu::channel::ChannelType;
use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
use std::collections::HashMap;
use std::error::Error;

pub mod channel;

mod osc;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MotuCommand {
    EnableMonitoring,
    DisableMonitoring,
    PrintSettings,
    Volume {
        channel: Channel,
        volume: f32,
    },
    Send {
        channel: Channel,
        aux_channel: Channel,
        value: f32,
    },
    Mute(Channel),
    Unmute(Channel),
    Init,
}
impl MotuCommand {
    fn hash_map(&self) -> Option<HashMap<String, String>> {
        let mut map = HashMap::new();
        match self {
            MotuCommand::EnableMonitoring => {
                return None;
            }
            MotuCommand::DisableMonitoring => {
                return None;
            }
            MotuCommand::PrintSettings => {
                return None;
            }
            MotuCommand::Volume { channel, volume } => {
                map.insert(
                    format!("/ch/{}/mix/level", channel.channel_number()),
                    volume.to_string(),
                );
            }
            MotuCommand::Send {
                channel,
                aux_channel,
                value,
            } => {
                map.insert(
                    format!(
                        "/ch/{}/mix/aux/{}/level",
                        channel.channel_number(),
                        aux_channel.channel_number()
                    ),
                    value.to_string(),
                );
            }
            MotuCommand::Mute(channel) => {
                map.insert(
                    format!("/ch/{}/mix/on", channel.channel_number()),
                    "0".to_string(),
                );
            }
            MotuCommand::Unmute(channel) => {
                map.insert(
                    format!("/ch/{}/mix/on", channel.channel_number()),
                    "1".to_string(),
                );
            }
            MotuCommand::Init => {
                return None;
            }
        }
        Some(map)
    }
}

pub struct Motu {
    client: osc::OscClient,
    aux_channels: HashMap<usize, String>,
    channels: HashMap<usize, String>,
    monitor_groups: HashMap<usize, String>,
}

impl Motu {
    pub fn new(ip: &str, port: &str, config: &Config) -> Result<Motu, Box<dyn Error>> {
        let client = osc::OscClient::new(&format!("{}:{}", ip, port))?;
        Ok(Motu {
            client,
            aux_channels: config.aux_channels.clone(),
            channels: config.channels.clone(),
            monitor_groups: config.monitor_groups.clone(),
        })
    }

    pub fn run(&self, commands: Vec<MotuCommand>) -> Result<(), Box<dyn Error>> {
        commands
            .into_iter()
            .flat_map(|command| self.process_commands(command))
            .filter(|command| command.hash_map().is_some())
            .for_each(|command| {
                self.send(command).unwrap();
            });
        Ok(())
    }

    fn process_commands(&self, command: MotuCommand) -> Vec<MotuCommand> {
        let mut commands: Vec<MotuCommand> = vec![];
        match command {
            MotuCommand::PrintSettings => {
                let _ = self.print_settings();
                // println!("aux_channels: {:?}", self.aux_channels);
                // println!("channels: {:?}", self.channels);
                // println!("monitor_groups: {:?}", self.monitor_groups);
            }
            MotuCommand::EnableMonitoring => {
                for group_index in self.monitor_groups.keys() {
                    commands.push(MotuCommand::Unmute(Channel::new(
                        *group_index as i32,
                        ChannelType::Group,
                    )));
                }
            }
            MotuCommand::DisableMonitoring => {
                for group_index in self.monitor_groups.keys() {
                    commands.push(MotuCommand::Mute(Channel::new(
                        *group_index as i32,
                        ChannelType::Group,
                    )));
                }
            }
            MotuCommand::Init => {
                self.channels.iter().for_each(|(channel_index, _)| {
                    let command = MotuCommand::Volume {
                        channel: Channel::new(*channel_index as i32, ChannelType::Chan),
                        volume: 1.0,
                    };
                    commands.push(command);
                    self.aux_channels.iter().for_each(|(aux_channel_index, _)| {
                        let command = MotuCommand::Send {
                            channel: Channel::new(*channel_index as i32, ChannelType::Chan),
                            aux_channel: Channel::new(*aux_channel_index as i32, ChannelType::Aux),
                            value: 0.0,
                        };
                        commands.push(command);
                    });
                });
                self.monitor_groups.iter().for_each(|(group_index, _)| {
                    let command =
                        MotuCommand::Unmute(Channel::new(*group_index as i32, ChannelType::Group));
                    commands.push(command);
                });
            }
            _ => {
                commands.push(command);
            }
        }
        commands
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
        let message = match command.hash_map() {
            Some(message) => message,
            None => return Err("No message found".into()),
        };
        message.iter().for_each(|(key, value)| {
            println!("{}: {}", key, value);
            let message = OscMessage::new(key, value.parse::<f32>().unwrap_or_default());
            let packet = OscPacket::Message(message);
            let _ = self.client.send(packet);
        });
        Ok(())
    }
}
