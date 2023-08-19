use crate::config::Config;
use crate::motu::channel::Channel;
use crate::motu::channel::ChannelType;
use reqwest::{
    blocking::{Client, Response},
    header,
};
use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
use std::collections::HashMap;
use std::error::Error;

pub use self::motucommand::MotuCommand;

pub mod channel;
pub mod motucommand;

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

pub struct Motu {
    http_client_url: String,
    client: osc::OscClient,
    aux_channels: HashMap<usize, String>,
    channels: HashMap<usize, String>,
    monitor_groups: HashMap<usize, String>,
}

impl Motu {
    pub fn new(ip: &str, port: &str, config: &Config) -> Result<Motu, Box<dyn Error>> {
        let client = osc::OscClient::new(&format!("{}:{}", ip, port))?;
        let http_client_url = format!("http://{}/datastore", ip);
        Ok(Motu {
            http_client_url,
            client,
            aux_channels: config.aux_channels.clone(),
            channels: config.channels.clone(),
            monitor_groups: config.monitor_groups.clone(),
        })
    }
    // runs the vector of commands
    pub fn run(&self, commands: &[MotuCommand]) -> Result<(), Box<dyn Error>> {
        let commands: Vec<MotuCommand> = commands
            .iter()
            .flat_map(|command| self.process_commands(command))
            .filter(|command| command.hash_map().is_some())
            .collect();
        if commands.len() >= 10 {
            let client = Client::new();

            let payload = {
                let long_string = commands
                    .iter()
                    .filter_map(|command| command.hash_map())
                    .map(|hash_map| {
                        hash_map
                            .iter()
                            .map(|(k, v)| format!("\"{}\": {}", k.replace("/mix/", "mix/"), v))
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", long_string)
            };

            // let payload = "{\"mix/chan/31/matrix/fader\": \"0.51410246\", \"mix/chan/2/matrix/fader\": \"0.51410246\" }";

            let mut headers = header::HeaderMap::new();
            headers.insert(
                "Content-Type",
                "application/x-www-form-urlencoded".parse().unwrap(),
            );
            // println!("json data: {}", payload);
            // Send the POST request with JSON payload
            let response: Response = client
                .post(&self.http_client_url)
                .headers(headers)
                .body(format!("json={}", payload))
                .send()?;
            // Check if the request was successful
            if !response.status().is_success() {
                println!("Request failed with status: {}", response.status());
            }
        } else {
            commands
                .into_iter()
               .for_each(|command| {
                    self.send(command).unwrap();
                });
        }
        Ok(())
    }

    fn process_commands(&self, command: &MotuCommand) -> Vec<MotuCommand> {
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
                commands.push(*command);
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
