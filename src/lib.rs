pub mod config;
pub mod motu;

#[cfg(test)]
mod tests {
    use crate::{
        config::Config,
        motu::{Channel, ChannelType, Motu, MotuCommand},
    };
    use std::{collections::HashMap, error::Error};

    fn get_mock_config() -> Config {
        let mut mock_config = Config {
            ip_address: String::from("127.0.0.1:8000"),
            aux_channels: HashMap::new(),
            channels: HashMap::new(),
            monitor_groups: HashMap::new(),
        };
        mock_config.aux_channels.insert(1, String::from("Aux 1"));
        mock_config.channels.insert(1, String::from("Channel 1"));
        mock_config
            .monitor_groups
            .insert(1, String::from("Monitor Group 1"));
        mock_config
    }

    #[test]
    fn test_enable_monitoring() -> Result<(), Box<dyn Error>> {
        let mock_config = get_mock_config();
        let motu = Motu::new("127.0.0.1:8000", &mock_config)?;
        motu.send(MotuCommand::EnableMonitoring)?;
        // Add assertions here to check that monitoring is enabled
        Ok(())
    }

    #[test]
    fn test_disable_monitoring() -> Result<(), Box<dyn Error>> {
        let mock_config = get_mock_config();
        let motu = Motu::new("127.0.0.1:8000", &mock_config)?;
        motu.send(MotuCommand::DisableMonitoring)?;
        // Add assertions here to check that monitoring is disabled
        Ok(())
    }

    #[test]
    fn test_print_settings() -> Result<(), Box<dyn Error>> {
        let mock_config = get_mock_config();
        let motu = Motu::new("127.0.0.1:8000", &mock_config)?;
        motu.send(MotuCommand::PrintSettings)?;
        // Add assertions here to check that settings are printed
        Ok(())
    }

    #[test]
    fn test_volume() -> Result<(), Box<dyn Error>> {
        let mock_config = get_mock_config();
        let motu = Motu::new("127.0.0.1:8000", &mock_config)?;
        motu.send(MotuCommand::Volume {
            channel: Channel::new(1, ChannelType::Chan),
            volume: 1.0,
        })?;
        // Add assertions here to check that volume is set correctly
        Ok(())
    }

    #[test]
    fn test_send() -> Result<(), Box<dyn Error>> {
        let mock_config = get_mock_config();
        let motu = Motu::new("127.0.0.1:8000", &mock_config)?;
        motu.send(MotuCommand::Send(
            Channel::new(1, ChannelType::Chan),
            Channel::new(1, ChannelType::Aux),
            0.5,
        ))?;
        // Add assertions here to check that send is set correctly
        Ok(())
    }
}
