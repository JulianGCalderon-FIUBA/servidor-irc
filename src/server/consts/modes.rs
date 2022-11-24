pub const SET_OPERATOR: char = 'o';
pub const PRIVATE: char = 'p';
pub const SECRET: char = 's';
pub const INVITE_ONLY: char = 'i';
pub const TOPIC_SETTABLE: char = 't';
pub const NO_OUTSIDE_MESSAGES: char = 'n';
pub const MODERATED: char = 'm';
pub const SET_USER_LIMIT: char = 'l';
pub const SET_BANMASK: char = 'b';
pub const SET_SPEAKER: char = 'v';
pub const SET_KEY: char = 'k';

pub const OPERATOR_SYMBOL: char = '@';
pub const SPEAKER_SYMBOL: char = '+';

pub const VALID_MODES: [char; 11] = [
    SET_OPERATOR,
    PRIVATE,
    SECRET,
    INVITE_ONLY,
    TOPIC_SETTABLE,
    NO_OUTSIDE_MESSAGES,
    MODERATED,
    SET_USER_LIMIT,
    SET_BANMASK,
    SET_SPEAKER,
    SET_KEY,
];

pub const ADD_MODE: char = '+';
pub const REMOVE_MODE: char = '-';

// pub const OPERATOR: char = 'o';

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ChannelFlag {
    Private,
    Secret,
    InviteOnly,
    NoOutsideMessages,
    TopicByOperatorOnly,
    Moderated,
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
        }
    }
    pub fn from_char(mode: char) -> Self {
        match mode {
            PRIVATE => ChannelFlag::Private,
            SECRET => ChannelFlag::Secret,
            INVITE_ONLY => ChannelFlag::InviteOnly,
            TOPIC_SETTABLE => ChannelFlag::TopicByOperatorOnly,
            NO_OUTSIDE_MESSAGES => ChannelFlag::NoOutsideMessages,
            MODERATED => ChannelFlag::Moderated,
            _ => panic!("Invalid mode"),
        }
    }
}
