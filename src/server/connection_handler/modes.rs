pub const SET_OPERATOR: u8 = b'o';
pub const PRIVATE: u8 = b'p';
pub const SECRET: u8 = b's';
pub const INVITE_ONLY: u8 = b'i';
pub const TOPIC_SETTABLE: u8 = b't';
pub const NO_OUTSIDE_MESSAGES: u8 = b'n';
pub const MODERATED: u8 = b'm';
pub const SET_USER_LIMIT: u8 = b'l';
pub const SET_BANMASK: u8 = b'b';
pub const SET_SPEAKER: u8 = b'v';
pub const SET_KEY: u8 = b'k';

pub const VALID_MODES: [u8; 11] = [
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

pub const INVISIBLE: char = 'i';
pub const OPERATOR: char = 'o';
