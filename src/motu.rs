use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
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
    fn new(address: &str, args: Vec<OscType>) -> Self;
}

impl OscSender for OscMessage {
    fn new(address: &str, args: Vec<OscType>) -> OscMessage {
        OscMessage {
            addr: address.to_string(),
            args,
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
}

pub struct Motu {
    client: osc::OscClient,
}

impl Motu {
    pub fn new(ip_address: &str) -> Result<Motu, Box<dyn Error>> {
        let client = osc::OscClient::new(ip_address)?;
        Ok(Motu { client })
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
        self.client.send(OscPacket::Message(OscMessage::new("/info", vec![]))).unwrap();
    }

    pub fn enable_monitoring(&self) {
        // Code to enable monitoring goes here
        println!("Monitoring enabled");
    }

    pub fn disable_monitoring(&self) {
        // Code to disable monitoring goes here
        println!("Monitoring disabled");
    }

    pub fn print_settings(&self) {
        // Code to print current settings goes here
    }
    pub fn send(&self, command: MotuCommand) -> Result<(), Box<dyn Error>> {
        let message = match command {
            MotuCommand::EnableMonitoring => OscMessage::new("/enable_monitoring", vec![]),
            MotuCommand::DisableMonitoring => OscMessage::new("/disable_monitoring", vec![]),
            MotuCommand::PrintSettings => {
                // Code to create OscMessage for printing settings goes here
                OscMessage::new("/print_settings", vec![])
            }
            MotuCommand::Volume(channel, volume) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let address = format!("{}/Fader", channel_number.unwrap_or(0));
                
                OscMessage::new(&address, vec![OscType::Float(volume)])
            }
            MotuCommand::Send(channel, aux_channel, value) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let aux_channel_number = aux_channel.map(|c| c.number.unwrap_or(0));
                let address = format!(
                    "{}/Aux/{}/Fader",
                    channel_number.unwrap_or(0),
                    aux_channel_number.unwrap_or(0)
                );
                
                OscMessage::new(&address, vec![OscType::Float(value)])
            }
        };

        let packet = OscPacket::Message(message);
        self.client.send(packet)?;
        Ok(())
    }
}
