use std::error::Error;

// use motuman::config::Config;
use crate::config::Config;

mod osc;

enum Channel {
    Number(Option<i32>),
    Description(String),
    Stereo(Option<bool>),
}

enum MotuCommand {
    EnableMonitoring,
    DisableMonitoring,
    PrintSettings,
    Volume(Channel),
    Send(Channel,Channel)
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
}
