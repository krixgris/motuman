use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChannelType {
    Aux,
    Chan,
    Group,
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let channel_type = match self {
            ChannelType::Aux => "aux",
            ChannelType::Chan => "chan",
            ChannelType::Group => "group",
        };
        write!(f, "{}", channel_type)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Channel {
    channel_number: i32,
    channel_type: ChannelType,
}

impl Channel {
    pub fn new(arg: i32, channel_type: ChannelType) -> Channel {
        Channel {
            channel_number: arg,
            channel_type,
        }
    }

    pub fn channel_number(&self) -> i32 {
        self.channel_number
    }

    pub fn channel_type(&self) -> &ChannelType {
        &self.channel_type
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{type}({number})", type = self.channel_type, number = self.channel_number)
    }
}