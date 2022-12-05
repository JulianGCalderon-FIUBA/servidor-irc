use super::channel::{DISTRIBUTED_CHANNEL, LOCAL_CHANNEL};

pub const INVALID_NICKNAME_CHARACTERS: [char; 7] = [' ', ',', '*', '?', '!', '@', '.'];

pub const INVALID_NICKNAME_PREFIXES: [char; 4] = ['$', ':', DISTRIBUTED_CHANNEL, LOCAL_CHANNEL];
