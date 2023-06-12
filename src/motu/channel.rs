use std::fmt::Display;

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