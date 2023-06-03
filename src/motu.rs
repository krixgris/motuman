use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
use std::collections::HashMap;
use std::error::Error;
// use motuman::config::Config;
use crate::config::Config;

mod osc;

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

// pub fn (address: &str, args: Vec<OscType>) -> OscMessage {
//     OscMessage {
//         addr: address.to_string(),
//         args,
//     }
// }

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

    pub fn run(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        if config.monitor {
            self.enable_monitoring();
        } else {
            self.disable_monitoring();
        }

        // Rest of the code goes here

        self.test_osc();

        Ok(())
    }

    pub fn test_osc(&self) {
        self.client
            .send(OscPacket::Message(OscMessage::new("/info", 1.0)))
            .unwrap();
        // print all channels
        for (k, v) in &self.channels {
            println!("{}: {}", k, v);
        }
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
        // Code to disable monitoring goes here
        // should send a vec of motucommands with value 0.0 to disable monitoring
        // vec should be all channels that need to be 0.0
        println!("Monitoring disabled");
        Ok(())
    }

    pub fn print_settings(&self) {
        // Code to print current settings goes here
    }
    pub fn send(&self, command: MotuCommand) -> Result<(), Box<dyn Error>> {
        let message = match command {
            MotuCommand::EnableMonitoring => OscMessage::new("/enable_monitoring", 1.0),
            MotuCommand::DisableMonitoring => OscMessage::new("/disable_monitoring", 0.0),
            MotuCommand::PrintSettings => {
                // Code to create OscMessage for printing settings goes here
                OscMessage::new("/print_settings", 1.0)
            }
            MotuCommand::Volume(channel, volume) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let address = format!("{}/Fader", channel_number.unwrap_or(0));

                OscMessage::new(&address, volume)
            }
            MotuCommand::Send(channel, aux_channel, value) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let aux_channel_number = aux_channel.map(|c| c.number.unwrap_or(0));
                let address = format!(
                    "{}/Aux/{}/Fader",
                    channel_number.unwrap_or(0),
                    aux_channel_number.unwrap_or(0)
                );

                OscMessage::new(&address, value)
            }
            MotuCommand::Mute(channel) => {
                // Code to create OscMessage for muting channel goes here
                // OscMessage::new("/mute", 1.0)

                // this should follow this structure: 4/matrix/mute where 4 is the channel number
                // use channel number from Channel struct into new variable called channel_number
                // then use channel_number in the OscMessage
                // OscMessage::new("/4/matrix/mute", 1.0)
                let channel_number = channel.map(|c| c.number.unwrap_or(0));

                let address = format!("{}/matrix/mute", channel_number.unwrap_or(0));
                OscMessage::new(&address, 1.0)
            }
            MotuCommand::Unmute(channel) => {
                // Code to create OscMessage for unmuting channel goes here
                // OscMessage::new("/unmute", 1.0)

                // this should follow this structure: 4/matrix/mute where 4 is the channel number
                // use channel number from Channel struct into new variable called channel_number
                // then use channel_number in the OscMessage
                // OscMessage::new("/4/matrix/mute", 0.0)
                let channel_number = channel.map(|c| c.number.unwrap_or(0));

                let address = format!("{}/matrix/mute", channel_number.unwrap_or(0));
                OscMessage::new(&address, 0.0)
            }
        };

        let packet = OscPacket::Message(message);
        self.client.send(packet)?;
        Ok(())
    }
}
