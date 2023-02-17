mod dcc_chat;
mod dcc_message;
mod dcc_send;

use crate::{macros::some_or_return, message::Message, server::consts::commands::PRIVMSG_COMMAND};

pub const CONTROL_CHARACTER: char = '\x01';

pub fn parse_ctcp(message: &Message) -> Option<String> {
    let command = message.get_command();
    if command != PRIVMSG_COMMAND {
        return None;
    }

    let mut trailing: Vec<char> = message.get_trailing().to_owned().unwrap().chars().collect();

    let first = some_or_return!(trailing.first(), None);
    let last = some_or_return!(trailing.last(), None);

    if first != &CONTROL_CHARACTER || last != &CONTROL_CHARACTER {
        return None;
    }

    trailing.remove(0);
    trailing.pop();

    Some(trailing.into_iter().collect())
}
