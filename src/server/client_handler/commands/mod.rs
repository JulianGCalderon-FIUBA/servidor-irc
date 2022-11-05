use crate::server::ClientHandler;

pub mod channel_operations;
pub mod connection_registration;
pub mod sending_messages;
pub mod user_based_queries;
mod utils;

#[cfg(test)]
mod tests;

pub const INVALID_CHARACTER: char = '\'';
pub const MAX_CHANNELS: usize = 10;

pub const DISTRIBUTED_CHANNEL: u8 = b'#';
pub const LOCAL_CHANNEL: u8 = b'&';

pub const WHOIS_COMMAND: &str = "WHOIS";
pub const WHO_COMMAND: &str = "WHO";
pub const NOTICE_COMMAND: &str = "NOTICE";
pub const PRIVMSG_COMMAND: &str = "PRIVMSG";
pub const NICK_COMMAND: &str = "NICK";
pub const OPER_COMMAND: &str = "OPER";
pub const PASS_COMMAND: &str = "PASS";
pub const QUIT_COMMAND: &str = "QUIT";
pub const USER_COMMAND: &str = "USER";
pub const INVITE_COMMAND: &str = "INVITE";
pub const JOIN_COMMAND: &str = "JOIN";
pub const LIST_COMMAND: &str = "LIST";
pub const NAMES_COMMAND: &str = "NAMES";
pub const PART_COMMAND: &str = "PART";
