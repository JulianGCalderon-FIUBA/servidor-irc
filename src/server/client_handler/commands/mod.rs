use crate::server::ClientHandler;
/// This module contains operations regarding channels.
pub mod channel_operations;
/// This module contains operations regarding the registration process.
pub mod connection_registration;
/// This module contains operations regarding the sending messages process.
pub mod sending_messages;
/// This module contains operations regarding the user based queries.
pub mod user_based_queries;
/// This module contains extra functionalities that can be useful when working with commands.
mod utils;

#[cfg(test)]
mod tests;

pub const INVALID_CHARACTER: char = '\'';
pub const MAX_CHANNELS: usize = 10;

pub const DISTRIBUTED_CHANNEL: u8 = b'#';
pub const LOCAL_CHANNEL: u8 = b'&';

pub const PASS_COMMAND: &str = "PASS";
pub const NICK_COMMAND: &str = "NICK";
pub const USER_COMMAND: &str = "USER";
pub const QUIT_COMMAND: &str = "QUIT";

pub const OPER_COMMAND: &str = "OPER";

pub const INVITE_COMMAND: &str = "INVITE";

pub const PRIVMSG_COMMAND: &str = "PRIVMSG";
pub const NOTICE_COMMAND: &str = "NOTICE";

pub const JOIN_COMMAND: &str = "JOIN";
pub const PART_COMMAND: &str = "PART";
pub const TOPIC_COMMAND: &str = "TOPIC";

pub const LIST_COMMAND: &str = "LIST";
pub const NAMES_COMMAND: &str = "NAMES";

pub const WHOIS_COMMAND: &str = "WHOIS";
pub const WHO_COMMAND: &str = "WHO";
