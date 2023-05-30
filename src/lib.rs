pub mod motu;
pub mod config;

#[cfg(test)]
mod tests {
    use crate::motu::{Channel, MotuCommand, Motu};

    use super::*;
    use std::error::Error;

    #[test]
    fn test_enable_monitoring() -> Result<(), Box<dyn Error>> {
        let motu = Motu::new("127.0.0.1:1234")?;
        motu.send(MotuCommand::EnableMonitoring)?;
        // Add assertions here to check that monitoring is enabled
        Ok(())
    }

    #[test]
    fn test_disable_monitoring() -> Result<(), Box<dyn Error>> {
        let motu = Motu::new("127.0.0.1:1234")?;
        motu.send(MotuCommand::DisableMonitoring)?;
        // Add assertions here to check that monitoring is disabled
        Ok(())
    }

    #[test]
    fn test_print_settings() -> Result<(), Box<dyn Error>> {
        let motu = Motu::new("127.0.0.1:1234")?;
        motu.send(MotuCommand::PrintSettings)?;
        // Add assertions here to check that settings are printed
        Ok(())
    }

    #[test]
    fn test_volume() -> Result<(), Box<dyn Error>> {
        let motu = Motu::new("127.0.0.1:1234")?;
        motu.send(MotuCommand::Volume(Some(Channel::new(1)), 0.5))?;
        // Add assertions here to check that volume is set correctly
        Ok(())
    }

    #[test]
    fn test_send() -> Result<(), Box<dyn Error>> {
        let motu = Motu::new("127.0.0.1:8000")?;
        motu.send(MotuCommand::Send(Some(Channel::new(1)), Some(Channel::new(2)), 0.5))?;
        // Add assertions here to check that send is set correctly
        Ok(())
    }
}