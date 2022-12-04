use std::collections::HashMap;

use crate::{
    message::Message,
    server::consts::commands::{INVITE_COMMAND, PRIVMSG_COMMAND},
};

use super::controller_message::ControllerMessage;

pub const LIST_RPL_COMMAND: &str = "322";
pub const END_LIST_RPL_COMMAND: &str = "323";
pub const NAMES_RPL_COMMAND: &str = "353";
pub const END_NAMES_RPL_COMMAND: &str = "366";

static mut CHANNELS_LIST_COMMAND: Vec<String> = vec![];
static mut CHANNELS_NAMES_COMMAND: Vec<String> = vec![];
static mut CLIENTS_NAMES_COMMAND: Vec<Vec<String>> = vec![];

pub fn to_controller_message(message: Message) -> ControllerMessage {
    println!("el mensaje es: {}", message);

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
        INVITE_COMMAND => ControllerMessage::RecieveInvite {
            nickname: message.get_prefix().clone().unwrap(),
            channel: message.get_parameters()[1].clone(),
        },
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
        _ => ControllerMessage::RegularMessage {
            message: message.to_string(),
        },
    }
}

pub fn is_channel(parameter: String) -> bool {
    parameter.starts_with('#')
}
