use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
use rosc::address;
use std::collections::HashMap;
use std::error::Error;
// use motuman::config::Config;
use crate::config::Config;

mod osc;

/// constant also needs the ip without port to be used in the http server
const HTTP_PREFIX: &str = "/datastore";

pub struct Channel {
    number: Option<i32>,
    description: String,
    stereo: Option<bool>,
}
impl Channel {
    pub fn new(arg: i32) -> Channel {
        Channel {
            number: Some(arg),
            description: String::from(""),
            stereo: None,
        }
    }
}

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
    Volume(Option<Channel>, f32),
    Send(Option<Channel>, Option<Channel>, f32),
    Mute(Option<Channel>),
    Unmute(Option<Channel>),
}

pub struct Motu {
    client: osc::OscClient,
    aux_channels: HashMap<String, usize>,
    channels: HashMap<String, usize>,
    monitor_groups: HashMap<String, usize>,
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
            self.send(command)?;
        }

        self.test_osc();

        Ok(())
    }

    pub fn test_osc(&self) {
        let mut keys: Vec<_> = self.channels.keys().collect();
        keys.sort();
        for key in keys {
         println!("{}: {}", key, self.channels[key]);
        }
        // for (k, v) in &self.channels {
        //     println!("{}: {}", k, v);
        // }
        // print all aux channels
        for (k, v) in &self.aux_channels {
            println!("{}: {}", k, v);
        }
        // print all monitor groups
        for (k, v) in &self.monitor_groups {
            println!("{}: {}", k, v);
        }
    }

    pub fn enable_monitoring(&self) -> Result<(), Box<dyn Error>> {
        for (group_name, group_index) in &self.monitor_groups {
            println!("Enabling monitoring for {} {}", group_index, group_name);
            let command = MotuCommand::Unmute(Some(Channel::new(*group_index as i32)));
            self.send(command)?;
        }
        println!("Monitoring enabled");
        Ok(())
    }

    pub fn disable_monitoring(&self) -> Result<(), Box<dyn Error>> {
        for (group_name, group_index) in &self.monitor_groups {
            println!("Disabling monitoring for {} {}", group_index, group_name);
            let command = MotuCommand::Mute(Some(Channel::new(*group_index as i32)));
            self.send(command)?;
        }
        println!("Monitoring disabled");
        Ok(())
    }

    pub fn print_settings(&self) {
        // Code to print current settings goes here
    }
    pub fn send(&self, command: MotuCommand) -> Result<(), Box<dyn Error>> {
        let message = match command {
            MotuCommand::EnableMonitoring => return self.enable_monitoring(),
            MotuCommand::DisableMonitoring => return self.disable_monitoring(),
            MotuCommand::PrintSettings => OscMessage::new("/print_settings", 1.0),
            MotuCommand::Volume(channel, volume) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                // format should be /mix/chan/<channel_number>/matrix/fader":1.0,
                let address = format!("/mix/chan/{}/matrix/fader", channel_number.unwrap_or(0));
                // // let address = format!(, channel_number.unwrap_or(0));

                OscMessage::new(&address, volume)
            }
            MotuCommand::Send(channel, aux_channel, value) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let aux_channel_number = aux_channel.map(|c| c.number.unwrap_or(0));
                // the format should be /mix/chan/<chnanel_number>/matrix/aux/<aux_channel_number>/send
                let address = format!(
                    "/mix/chan/{}/matrix/aux/{}/send",
                    channel_number.unwrap_or(0),
                    aux_channel_number.unwrap_or(0)
                );
                // let address = format!(
                //     "{}/Aux/{}/Fader",
                //     channel_number.unwrap_or(0),
                //     aux_channel_number.unwrap_or(0)
                // );

                OscMessage::new(&address, value)
            }
            MotuCommand::Mute(channel) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let address = format!("/mix/group/{}/matrix/mute", channel_number.unwrap_or(0));
                OscMessage::new(&address, 1.0)
            }
            MotuCommand::Unmute(channel) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let address = format!("/mix/group/{}/matrix/mute", channel_number.unwrap_or(0));
                OscMessage::new(&address, 0.0)
            }
        };

        let packet = OscPacket::Message(message);
        self.client.send(packet)?;
        Ok(())
    }
}
