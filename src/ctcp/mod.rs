use crate::{macros::some_or_return, message::Message, server::consts::commands::PRIVMSG_COMMAND};

pub mod dcc_chat;
pub mod dcc_message;

pub const CONTROL_CHARACTER: char = 1 as char;

pub fn is_ctcp_message(message: &Message) -> bool {
    let command = message.get_command();
    let trailing: Vec<char> = message.get_trailing().as_ref().unwrap().chars().collect();

    if command != PRIVMSG_COMMAND {
        return false;
    }

    let first = some_or_return!(trailing.first(), false);
    let last = some_or_return!(trailing.last(), false);

    first == &CONTROL_CHARACTER && last == &CONTROL_CHARACTER
}

pub fn get_ctcp_message(message: &Message) -> Option<String> {
    if !is_ctcp_message(message) {
        return None;
    }

    let mut content = message.get_trailing().to_owned().unwrap();

    content.remove(0);
    content.pop();

    Some(content)
}
