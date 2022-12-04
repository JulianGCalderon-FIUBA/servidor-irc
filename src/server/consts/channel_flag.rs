use super::modes::*;

#[derive(PartialEq, Eq, Clone, Debug)]

/// Possible flags a channel may have
pub enum ChannelFlag {
    Private,
    Secret,
    InviteOnly,
    NoOutsideMessages,
    TopicByOperatorOnly,
    Moderated,
    InvalidFlag,
}

impl ChannelFlag {
    pub fn to_char(&self) -> char {
        match self {
            ChannelFlag::Private => PRIVATE,
            ChannelFlag::Secret => SECRET,
            ChannelFlag::InviteOnly => INVITE_ONLY,
            ChannelFlag::NoOutsideMessages => NO_OUTSIDE_MESSAGES,
            ChannelFlag::TopicByOperatorOnly => TOPIC_SETTABLE,
            ChannelFlag::Moderated => MODERATED,
            ChannelFlag::InvalidFlag => panic!("Flag is invalid"),
        }
    }
    pub fn from_char(character: char) -> Self {
        match character {
            PRIVATE => ChannelFlag::Private,
            SECRET => ChannelFlag::Secret,
            INVITE_ONLY => ChannelFlag::InviteOnly,
            TOPIC_SETTABLE => ChannelFlag::TopicByOperatorOnly,
            NO_OUTSIDE_MESSAGES => ChannelFlag::NoOutsideMessages,
            MODERATED => ChannelFlag::Moderated,
            _ => ChannelFlag::InvalidFlag,
        }
    }
}
