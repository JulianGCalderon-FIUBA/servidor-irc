use crate::{
    message::Message,
    server::client_handler::commands::{INVITE_COMMAND, PRIVMSG_COMMAND},
};

use super::controller_message::ControllerMessage;

pub const LIST_RPL_COMMAND: &str = "353";
pub const END_LIST_RPL_COMMAND: &str = "323";

static mut CHANNELS: Vec<String> = vec![];

pub fn to_controller_message(message: Message) -> ControllerMessage {
    // commands with no ControllerMessage
    if &message.get_command()[..] == LIST_RPL_COMMAND {
        unsafe {
            CHANNELS.push(message.get_parameters()[0].clone());
        }
    }

    // commands that return ControllerMessage
    match &message.get_command()[..] {
        PRIVMSG_COMMAND => ControllerMessage::ReceivePrivMessage {
            nickname: message.get_prefix().clone().unwrap(),
            message: message.get_trailing().clone().unwrap(),
        },
        INVITE_COMMAND => ControllerMessage::RecieveInvite {
            nickname: message.get_prefix().clone().unwrap(),
            channel: message.get_parameters()[1].clone(),
        },
        END_LIST_RPL_COMMAND => unsafe {
            let channels_clone = CHANNELS.clone();
            CHANNELS = vec![];
            ControllerMessage::ReceiveListChannels {
                channels: channels_clone,
            }
        },
        _ => ControllerMessage::RegularMessage {
            message: message.to_string(),
        },
    }
}
