use crate::server::ClientHandler;

pub(crate) mod channel_operations;
pub(crate) mod connection_registration;
pub(crate) mod sending_messages;
mod utils;

pub const INVALID_CHARACTER: char = '\'';
pub const MAX_CHANNELS: usize = 10;

pub const DISTRIBUTED_CHANNEL: u8 = b'#';
pub const LOCAL_CHANNEL: u8 = b'&';
