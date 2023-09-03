// motucommand from motu.rs goes here
// Path: src/motu/motu.rs

use super::channel::{Channel, ChannelType};
use std::fmt::Display;

const MAX_AUX_CHANNELS: usize = 8;

#[derive(Debug, PartialEq, Clone)]
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
    ToggleSend {
        channel: Channel,
        aux_channels: Vec<Channel>,
        active_sends: u8,
    },
}
impl MotuCommand {
    pub fn http_command(&self) -> Option<String> {
        let endpoint_value: Option<String> = match self {
            MotuCommand::EnableMonitoring => {
                return None;
            }
            MotuCommand::DisableMonitoring => {
                return None;
            }
            MotuCommand::PrintSettings => {
                return None;
            }
            MotuCommand::Volume { channel, volume } => Some(format!(
                "\"mix/{}/{}/matrix/fader\":{}",
                channel.channel_type(),
                channel.channel_number(),
                volume
            )),
            MotuCommand::Send {
                channel,
                aux_channel,
                value,
            } => Some(format!(
                "\"mix/{}/{}/matrix/aux/{}/send\":{}",
                channel.channel_type(),
                channel.channel_number(),
                aux_channel.channel_number(),
                value
            )),
            MotuCommand::Mute(channel) => Some(format!(
                "\"mix/{}/{}/matrix/mute\":1",
                channel.channel_type(),
                channel.channel_number()
            )),
            MotuCommand::Unmute(channel) => Some(format!(
                "\"mix/{}/{}/matrix/mute\":0",
                channel.channel_type(),
                channel.channel_number()
            )),
            MotuCommand::Init => {
                return None;
            }
            MotuCommand::ToggleSend {
                channel: _,
                aux_channels: _,
                active_sends: _,
            } => {
                todo!()
            }
        };

        endpoint_value
    }
    pub fn osc_command(&self) -> Option<(String, String)> {
        let osc_command: Option<(String, String)> = match self {
            MotuCommand::EnableMonitoring => {
                return None;
            }
            MotuCommand::DisableMonitoring => {
                return None;
            }
            MotuCommand::PrintSettings => {
                return None;
            }
            MotuCommand::Volume { channel, volume } => Some((
                format!(
                    "/mix/{}/{}/matrix/fader",
                    channel.channel_type(),
                    channel.channel_number()
                ),
                volume.to_string(),
            )),
            MotuCommand::Send {
                channel,
                aux_channel,
                value,
            } => Some((
                format!(
                    "/mix/{}/{}/matrix/aux/{}/send",
                    channel.channel_type(),
                    channel.channel_number(),
                    aux_channel.channel_number()
                ),
                value.to_string(),
            )),
            MotuCommand::Mute(channel) => Some((
                format!(
                    "/mix/{}/{}/matrix/mute",
                    channel.channel_type(),
                    channel.channel_number()
                ),
                "1".to_string(),
            )),
            MotuCommand::Unmute(channel) => Some((
                format!(
                    "/mix/{}/{}/matrix/mute",
                    channel.channel_type(),
                    channel.channel_number()
                ),
                "0".to_string(),
            )),
            MotuCommand::Init => {
                return None;
            }
            MotuCommand::ToggleSend {
                channel: _,
                aux_channels: _,
                active_sends: _,
            } => {
                return None;
            }
        };
        osc_command
    }
    pub fn set_value(&mut self, new_value: f32) {
        match self {
            MotuCommand::Volume { channel: _, volume } => *volume = new_value,

            MotuCommand::Send {
                channel: _,
                aux_channel: _,
                value,
            } => *value = new_value,
            MotuCommand::ToggleSend {
                channel: _,
                aux_channels,
                active_sends,
            } => {
                let mut mask: u8 = 0b1111_1111;
                let channel_count = aux_channels.len();
                mask >>= MAX_AUX_CHANNELS - channel_count;
                *active_sends += 1;
                if active_sends > &mut mask {
                    *active_sends = 0;
                }
            }
            _ => (),
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
                } else if send.len() < MAX_AUX_CHANNELS {
                    let (channel, aux_channel, aux2) = (
                        send[0].parse::<i32>(),
                        send[1].parse::<i32>(),
                        send[2].parse::<i32>(),
                    );
                    match (channel, aux_channel, aux2) {
                        (Ok(channel), Ok(_aux_channel), Ok(_aux2)) => {
                            MotuCommand::ToggleSend {
                                                    channel: Channel::new(channel, ChannelType::Chan),
                                                    aux_channels: //vec![Channel::new(aux_channel, ChannelType::Aux), Channel::new(aux2, ChannelType::Aux)],
                                                    {
                                                        send.iter()
                                                            .skip(1) // Skip the first element
                                                            .map(|s| s.parse::<i32>().map(|aux_channel| Channel::new(aux_channel, ChannelType::Aux)))
                                                            .filter_map(Result::ok)
                                                            .collect()
                                                    },
                                                    active_sends: 0,
                                                }
                        }
                        _ => return Err("Invalid send".to_string()),
                    }
                } else {
                    return Err("Invalid send".to_string());
                }
            } else if s.contains("unmute") {
                let unmute = s.replace("unmute(", "").replace(')', "").parse::<i32>();
                match unmute {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            MotuCommand::ToggleSend {
                channel,
                aux_channels,
                active_sends,
            } => write!(f, "Send: {} {:#?} {}", channel, aux_channels, active_sends),
        }
    }
}
