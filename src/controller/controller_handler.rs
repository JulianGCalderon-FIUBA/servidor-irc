use std::collections::HashMap;

use crate::{
    message::Message,
    server::consts::commands::{INVITE_COMMAND, KICK_COMMAND, PRIVMSG_COMMAND},
};

use super::{controller_message::ControllerMessage, ERR_NICK_COLLISION_WARNING_TEXT};

const CHANNEL_FIRST_CHARACTER: &str = "#";

pub const LOGIN_OK: &str = "001";
pub const LIST_RPL_COMMAND: &str = "322";
pub const END_LIST_RPL_COMMAND: &str = "323";
pub const NAMES_RPL_COMMAND: &str = "353";
pub const END_NAMES_RPL_COMMAND: &str = "366";
pub const ERR_NICK_COLLISION: &str = "436";

static mut CHANNELS_LIST_COMMAND: Vec<String> = vec![];
static mut CHANNELS_NAMES_COMMAND: Vec<String> = vec![];
static mut CLIENTS_NAMES_COMMAND: Vec<Vec<String>> = vec![];

pub fn to_controller_message(message: Message) -> ControllerMessage {
    // commands with no ControllerMessage
    match &message.get_command()[..] {
        LIST_RPL_COMMAND => unsafe {
            CHANNELS_LIST_COMMAND.push(message.get_parameters()[0].clone());
        },
        NAMES_RPL_COMMAND => unsafe {
            CHANNELS_NAMES_COMMAND.push(message.get_parameters()[0].clone());
            let trailing: String = message.get_trailing().clone().unwrap();
            let current_clients: Vec<String> = trailing.split(' ').map(|s| s.to_string()).collect();
            CLIENTS_NAMES_COMMAND.push(current_clients);
        },
        _ => (),
    }

    // commands that return ControllerMessage
    match &message.get_command()[..] {
        END_LIST_RPL_COMMAND => unsafe {
            let channels_clone: Vec<String> = CHANNELS_LIST_COMMAND.clone();
            CHANNELS_LIST_COMMAND = vec![];
            ControllerMessage::ReceiveListChannels {
                channels: channels_clone,
            }
        },
        END_NAMES_RPL_COMMAND => unsafe {
            let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
            for i in 0..CHANNELS_NAMES_COMMAND.len() {
                hashmap.insert(
                    CHANNELS_NAMES_COMMAND[i].clone(),
                    CLIENTS_NAMES_COMMAND[i].clone(),
                );
            }
            CHANNELS_NAMES_COMMAND = vec![];
            CLIENTS_NAMES_COMMAND = vec![];
            ControllerMessage::ReceiveNamesChannels {
                channels_and_clients: hashmap,
            }
        },
        ERR_NICK_COLLISION => ControllerMessage::AddWarningView {
            message: ERR_NICK_COLLISION_WARNING_TEXT.to_string(),
        },
        INVITE_COMMAND => ControllerMessage::RecieveInvite {
            nickname: message.get_prefix().clone().unwrap(),
            channel: message.get_parameters()[1].clone(),
        },
        KICK_COMMAND => ControllerMessage::ReceiveKick {
            kicked: message.get_parameters()[1].clone(),
            channel: message.get_parameters()[0].clone(),
        },
        LOGIN_OK => {
            let trailing_text: String = message.get_trailing().clone().unwrap();
            let trailing_strings = trailing_text.split(' ').collect::<Vec<&str>>();
            println!("{:?}", trailing_strings);

            let mut username = trailing_strings[5].to_string();
            username.remove(0);
            ControllerMessage::ChangeViewToMain {
                realname: message.get_parameters()[0].clone(),
                servername: trailing_strings[2].to_string(),
                nickname: trailing_strings[4].to_string(),
                username,
            }
        }
        PRIVMSG_COMMAND => {
            let message_text = message.get_trailing().clone().unwrap();
            let sender_nickname = message.get_prefix().clone().unwrap();
            let channel = if is_channel(message.get_parameters()[0].clone()) {
                Some(message.get_parameters()[0].clone())
            } else {
                None
            };

            ControllerMessage::ReceivePrivMessage {
                sender_nickname,
                message: message_text,
                channel,
            }
        }
        _ => ControllerMessage::RegularMessage {
            message: message.to_string(),
        },
    }
}

pub fn is_channel(parameter: String) -> bool {
    parameter.starts_with(CHANNEL_FIRST_CHARACTER)
}
