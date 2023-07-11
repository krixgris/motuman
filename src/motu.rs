use crate::config::Config;
use crate::motu::channel::Channel;
use crate::motu::channel::ChannelType;
use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

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

    pub fn set_midi_value(self, midi_value: u8) -> MotuCommand {
        match self {
            MotuCommand::Volume { channel, volume: _ } => MotuCommand::Volume {
                channel,
                volume: midi_value as f32 / 127.0,
            },
            MotuCommand::Send {
                channel,
                aux_channel,
                value: _,
            } => MotuCommand::Send {
                channel,
                aux_channel,
                value: midi_value as f32 / 127.0,
            },
            _ => self,
        }
    }
}



impl std::str::FromStr for MotuCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let motu_command: MotuCommand = {
            if s.contains("vol") {
                let vol = s.replace("vol(", "").replace(')', "").parse::<i32>();
                match vol {
                    Ok(vol) => MotuCommand::Volume {
                        channel: Channel::new(vol, ChannelType::Chan),
                        volume: 0.66,
                    },
                    Err(_) => return Err("Invalid volume".to_string()),
                }
            } else if s.contains("send") {
                let send = s.replace("send(", "").replace(')', "");
                // if send contains 2 integers, it's a send command
                let send: Vec<&str> = send.split(',').collect();
                if send.len() == 2 {
                    let (channel, aux_channel) = (send[0].parse::<i32>(), send[1].parse::<i32>());
                    match (channel, aux_channel) {
                        (Ok(channel), Ok(aux_channel)) => MotuCommand::Send {
                            channel: Channel::new(channel, ChannelType::Chan),
                            aux_channel: Channel::new(aux_channel, ChannelType::Aux),
                            value: 0.33,
                        },
                        _ => return Err("Invalid send".to_string()),
                    }
                } else {
                    return Err("Invalid send".to_string());
                }
            } else if s.contains("unmute") {
                let unmute = s.replace("unmute(", "").replace(')', "").parse::<i32>();
                match dbg!(unmute) {
                    Ok(unmute) => MotuCommand::Unmute(Channel::new(unmute, ChannelType::Chan)),
                    Err(_) => return Err("Invalid unmute".to_string()),
                }
            } else if s.contains("mute") {
                let mute = s.replace("mute(", "").replace(')', "").parse::<i32>();
                match mute {
                    Ok(mute) => MotuCommand::Mute(Channel::new(mute, ChannelType::Chan)),
                    Err(_) => return Err("Invalid mute".to_string()),
                }
            } else if s.contains("monitor") {
                let monitor = s.replace("monitor(", "").replace(')', "");
                match monitor.as_str() {
                    "on" => MotuCommand::EnableMonitoring,
                    "off" => MotuCommand::DisableMonitoring,
                    _ => return Err("Invalid monitor".to_string()),
                }
            } else if s.contains("print") {
                MotuCommand::PrintSettings
            } else if s.contains("init") {
                MotuCommand::Init
            } else {
                return Err("Invalid command".to_string());
            }
        };
        Ok(motu_command)
    }
}

impl Display for MotuCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MotuCommand::EnableMonitoring => write!(f, "EnableMonitoring"),
            MotuCommand::DisableMonitoring => write!(f, "DisableMonitoring"),
            MotuCommand::PrintSettings => write!(f, "PrintSettings"),
            MotuCommand::Volume { channel, volume } => {
                write!(f, "Volume: {} {}", channel, volume)
            }
            MotuCommand::Send {
                channel,
                aux_channel,
                value,
            } => write!(f, "Send: {} {} {}", channel, aux_channel, value),
            MotuCommand::Mute(channel) => write!(f, "Mute: {}", channel),
            MotuCommand::Unmute(channel) => write!(f, "Unmute: {}", channel),
            MotuCommand::Init => write!(f, "Init"),
        }
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
            // println!("{}: {}", key, value);
            let message = OscMessage::new(key, value.parse::<f32>().unwrap_or_default());
            let packet = OscPacket::Message(message);
            let _ = self.client.send(packet);
        });
        Ok(())
    }
}
