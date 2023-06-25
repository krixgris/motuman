#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use crate::args::{self, Args, IpEndpoint};

    #[test]
    fn test_config_file_name() {
        let args = Args {
            config: String::from("test_config.toml"),
            monitor: None,
            channel: None,
            volume: None,
            ip_address: None,
            port: None,
            list_channels: false,
            aux_channel: None,
            send_amount: None,
            init: false,
        };
        assert_eq!(args.config_file_name(), "test_config.toml");
    }

    // #[test]
    // fn test_motu_commands() {
    //     let args = Args {
    //         config: String::from("test_config.toml"),
    //         monitor: Some(true),
    //         channel: Some(1),
    //         volume: Some(0.5),
    //         ip_address: Some(String::from("192.168.1.2")),
    //         port: Some(8000),
    //         list_channels: true,
    //         aux_channel: Some(2),
    //         send_amount: Some(0.3),
    //         init: true,
    //     };
    //     let mut expected_commands = vec![
    //         motu::MotuCommand::Init,
    //         motu::MotuCommand::EnableMonitoring,
    //         motu::MotuCommand::Send {
    //             channel: motu::channel::Channel::new(1, motu::channel::ChannelType::Chan),
    //             aux_channel: motu::channel::Channel::new(2, motu::channel::ChannelType::Aux),
    //             value: 0.3,
    //         },
    //         motu::MotuCommand::Volume {
    //             channel: motu::channel::Channel::new(1, motu::channel::ChannelType::Chan),
    //             volume: 0.5,
    //         },
    //         motu::MotuCommand::PrintSettings,
    //     ];
    //     assert_eq!(args.motu_commands(), expected_commands);

    //     let args = Args {
    //         config: String::from("test_config.toml"),
    //         monitor: Some(false),
    //         channel: Some(1),
    //         volume: Some(0.5),
    //         ip_address: Some(String::from("192.168.1.2")),
    //         port: Some(8000),
    //         list_channels: false,
    //         aux_channel: Some(2),
    //         send_amount: Some(0.3),
    //         init: false,
    //     };
    //     expected_commands = vec![
    //         motu::MotuCommand::DisableMonitoring,
    //         motu::MotuCommand::Send {
    //             channel: motu::channel::Channel::new(1, motu::channel::ChannelType::Chan),
    //             aux_channel: motu::channel::Channel::new(2, motu::channel::ChannelType::Aux),
    //             value: 0.3,
    //         },
    //         motu::MotuCommand::Volume {
    //             channel: motu::channel::Channel::new(1, motu::channel::ChannelType::Chan),
    //             volume: 0.5,
    //         },
    //     ];
    //     assert_eq!(args.motu_commands(), expected_commands);
    // }

    #[test]
    fn test_ip_address_only() {
        let args = Args {
            config: String::from("test_config.toml"),
            monitor: None,
            channel: None,
            volume: None,
            ip_address: Some(IpEndpoint::from("192.168.1.2")),
            port: Some(8000),
            list_channels: false,
            aux_channel: None,
            send_amount: None,
            init: false,
        };
        // assert_eq!(args.ip_address, Some(IpEndpoint::from("192.168.1.2")));

        let args = Args {
            config: String::from("test_config.toml"),
            monitor: None,
            channel: None,
            volume: None,
            ip_address: Some(IpEndpoint::from("192.168.1.2:9000")),
            port: Some(8000),
            list_channels: false,
            aux_channel: None,
            send_amount: None,
            init: false,
        };
        // assert_eq!(args.ip_address_only(), Some(IpEndpoint::from("192.168.1.2")));
    }
}