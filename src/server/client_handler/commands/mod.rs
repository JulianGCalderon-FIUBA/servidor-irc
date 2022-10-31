use crate::server::ClientHandler;

pub mod channel_operations;
pub mod connection_registration;
pub mod sending_messages;
pub mod user_based_queries;

#[cfg(test)]
mod tests;

pub const INVALID_CHARACTER: char = '\'';
pub const MAX_CHANNELS: usize = 10;

pub const DISTRIBUTED_CHANNEL: u8 = b'#';
pub const LOCAL_CHANNEL: u8 = b'&';
