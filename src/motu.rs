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
                // map.insert("/meters/monitoring/enabled".to_string(), "1".to_string());
                return None;
            }
            MotuCommand::DisableMonitoring => {
                // map.insert("/meters/monitoring/enabled".to_string(), "0".to_string());
                return None;
            }
            MotuCommand::PrintSettings => {
                // map.insert("/print".to_string(), "settings".to_string());
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
                // map.insert("/ch/1/mix/level".to_string(), "0.0".to_string());
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
        let special_commands: Vec<MotuCommand> = commands
            .into_iter()
            .map(|command| {
                let _ = self.send(command);
                command
                // if let Some(hash_map) = command.hash_map() {
                //     let mut new_commands: Vec<MotuCommand> = vec![];
                //     for (address, value) in hash_map {
                //         new_commands.push(MotuCommand::Osc {
                //             message: OscMessage::new(&address, value.parse::<f32>().unwrap()),
                //         });
                //     }
                //     return new_commands;
                // }
                // return vec![command];
            })
            .filter(|command| command.hash_map().is_none())
            .flat_map(|command| {
                {
                    let mut new_commands: Vec<MotuCommand> = vec![];
                    // command
                    match command {
                        MotuCommand::EnableMonitoring => {
                            for group_index in self.monitor_groups.keys() {
                                new_commands.push(MotuCommand::Unmute(Channel::new(
                                    *group_index as i32,
                                    ChannelType::Group,
                                )));
                            }
                            // new_commands
                            // .push(MotuCommand::Unmute(Channel::new(1, ChannelType::Group)));
                        }
                        MotuCommand::DisableMonitoring => {
                            for group_index in self.monitor_groups.keys() {
                                new_commands.push(MotuCommand::Mute(Channel::new(
                                    *group_index as i32,
                                    ChannelType::Group,
                                )));
                            }
                        }
                        MotuCommand::Init => {
                            // new_commands.push(MotuCommand::Volume {
                            //     channel: Channel::new(1, ChannelType::Group),
                            //     volume: 0.0,
                            // });
                            let mut commands = vec![];
                            // let delay: u64 = 2;
                            for (channel_index, channel_name) in &self.channels {
                                // println!(
                                //     "Setting volume for channel {} {}",
                                //     channel_index, channel_name
                                // );
                                let command = MotuCommand::Volume {
                                    channel: Channel::new(*channel_index as i32, ChannelType::Chan),
                                    volume: 1.0,
                                };
                                // std::thread::sleep(std::time::Duration::from_millis(delay));
                                commands.push(command);
                                for (aux_channel_index, aux_channel_name) in &self.aux_channels {
                                    // println!(
                                    //     "Setting send for channel {} {} to aux channel {} {}",
                                    //     channel_index, channel_name, aux_channel_index, aux_channel_name
                                    // );
                                    let command = MotuCommand::Send {
                                        channel: Channel::new(
                                            *channel_index as i32,
                                            ChannelType::Chan,
                                        ),
                                        aux_channel: Channel::new(
                                            *aux_channel_index as i32,
                                            ChannelType::Aux,
                                        ),
                                        value: 0.0,
                                    };
                                    // std::thread::sleep(std::time::Duration::from_millis(delay));
                                    commands.push(command);
                                }
                            }

                            for (group_index, group_name) in &self.monitor_groups {
                                // println!("Unmuting monitor group {} {}", group_index, group_name);
                                let command = MotuCommand::Unmute(Channel::new(
                                    *group_index as i32,
                                    ChannelType::Group,
                                ));
                                // std::thread::sleep(std::time::Duration::from_millis(delay));
                                commands.push(command);
                            }
                            new_commands.append(&mut commands);
                        }
                        _ => {
                            new_commands.push(command);
                        }
                    }
                    new_commands
                }
            })
            .collect();
        println!("Special commands: {:?}", special_commands);

        // let commands = commands.clone();
        // commands.append(&mut special_commands.clone());

        // for command in commands {
        //     // add a small pause of 40ms between commands?
        //     println!("Sending command: {:?}", command.hash_map());
        //     self.send(command)?;
        // }
        for command in special_commands {
            // add a small pause of 40ms between commands?
            println!("Sending command: {:?}", command.hash_map());
            self.send(command)?;
        }
        Ok(())
    }

    // pub fn enable_monitoring(&self) -> Result<(), Box<dyn Error>> {
    //     let commands: Vec<MotuCommand> = self
    //         .monitor_groups
    //         .keys()
    //         .map(|group_index| {
    //             MotuCommand::Unmute(Channel::new(*group_index as i32, ChannelType::Group))
    //         })
    //         .collect();
    //     println!("Enabling monitoring for {:?}", self.monitor_groups);
    //     self.run(commands)?;
    //     Ok(())
    // }

    // pub fn disable_monitoring(&self) -> Result<(), Box<dyn Error>> {
    //     let commands: Vec<MotuCommand> = self
    //         .monitor_groups
    //         .keys()
    //         .map(|group_index| {
    //             MotuCommand::Mute(Channel::new(*group_index as i32, ChannelType::Group))
    //         })
    //         .collect();
    //     println!("Disabling monitoring for {:?}", self.monitor_groups);
    //     self.run(commands)?;
    //     Ok(())
    // }

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
        // let message = match command {
        //     MotuCommand::EnableMonitoring => return self.enable_monitoring(),

        //     MotuCommand::DisableMonitoring => return self.disable_monitoring(),
        //     MotuCommand::PrintSettings => return self.print_settings(),
        //     MotuCommand::Volume { channel, volume } => {
        //         let (channel_number, channel_type) =
        //             (channel.channel_number() as usize, channel.channel_type());
        //         if !self.channels.contains_key(&channel_number) {
        //             return Err(format!("Channel {} is not defined", channel_number).into());
        //         }
        //         let address = format!("/mix/{}/{}/matrix/fader", channel_type, channel_number);

        //         OscMessage::new(&address, volume)
        //     }
        //     MotuCommand::Send {
        //         channel,
        //         aux_channel,
        //         value,
        //     } => {
        //         let (channel_number, aux_channel_number, channel_type) = (
        //             channel.channel_number() as usize,
        //             aux_channel.channel_number() as usize,
        //             channel.channel_type(),
        //         );
        //         if !self.channels.contains_key(&channel_number) {
        //             return Err(format!("Channel {} is not defined", channel_number).into());
        //         }
        //         if !self.aux_channels.contains_key(&aux_channel_number) {
        //             return Err(format!("Aux channel {} is not defined", aux_channel_number).into());
        //         }
        //         let address = format!(
        //             "/mix/{}/{}/matrix/aux/{}/send",
        //             channel_type, channel_number, aux_channel_number
        //         );

        //         OscMessage::new(&address, value)
        //     }
        //     MotuCommand::Mute(channel) => {
        //         let channel_number = channel.channel_number();
        //         let address = format!("/mix/group/{}/matrix/mute", channel_number);
        //         OscMessage::new(&address, 1.0)
        //     }
        //     MotuCommand::Unmute(channel) => {
        //         let channel_number = channel.channel_number();
        //         let address = format!("/mix/group/{}/matrix/mute", channel_number);
        //         OscMessage::new(&address, 0.0)
        //     }
        //     MotuCommand::Init => return self.init_settings(),
        // };

        // OscMessage::new(&address, 0.0)

        // create an OscMessage with the key and value from the hash_map() method on commands
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
        // let message = OscMessage::new(&command.hash_map().key(), command.hash_map().value());

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
                let command = MotuCommand::Send {
                    channel: Channel::new(*channel_index as i32, ChannelType::Chan),
                    aux_channel: Channel::new(*aux_channel_index as i32, ChannelType::Aux),
                    value: 0.0,
                };
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
