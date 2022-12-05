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

pub const VALID_CHANNEL_MODES: [char; 11] = [
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

pub const VALID_USER_MODES: [char; 4] = [
    INVISIBLE,
    OPERATOR,
    RECEIVES_SERVER_NOTICES,
    RECEIVES_WALLOPS,
];

pub const ADD_MODE: char = '+';
pub const REMOVE_MODE: char = '-';

pub const INVISIBLE: char = 'i';
pub const OPERATOR: char = 'o';
pub const RECEIVES_SERVER_NOTICES: char = 's';
pub const RECEIVES_WALLOPS: char = 'w';

pub const ADD_OPERATOR: &str = "+o";
pub const NO_TOPIC: &str = "No topic set";
