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
        self.client.send("/info").unwrap();
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
        match command {
            MotuCommand::EnableMonitoring => {
                self.enable_monitoring();
            }
            MotuCommand::DisableMonitoring => {
                self.disable_monitoring();
            }
            MotuCommand::PrintSettings => {
                self.print_settings();
            }
            MotuCommand::Volume(channel, volume) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                println!(
                    "Set volume {} on channel {}",
                    volume,
                    channel_number.unwrap_or(0)
                );
                // self.client.send("/volume", (channel_number, volume))?;
            }
            MotuCommand::Send(channel, aux_channel, value) => {
                let channel_number = channel.map(|c| c.number.unwrap_or(0));
                let aux_channel_number = aux_channel.map(|c| c.number.unwrap_or(0));
                let formatted_string = format!(
                    "channel({})/aux/send_channel({})/value({})",
                    channel_number.unwrap_or(0),
                    aux_channel_number.unwrap_or(0),
                    value
                );
                println!(
                    "Send {} to channel {} aux {}",
                    value,
                    channel_number.unwrap_or(0),
                    aux_channel_number.unwrap_or(0)
                );
                
                self.client.send(formatted_string.as_str())?;
                
            }
        }

        Ok(())
    }
}
