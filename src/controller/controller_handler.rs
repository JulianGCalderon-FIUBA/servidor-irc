use crate::message::Message;

use super::controller_message::ControllerMessage;

pub const PRIVMSG_COMMAND: &str = "PRIVMSG";
pub const INVITE_COMMAND: &str = "INVITE";
pub const LIST_COMMAND: &str = "353";
pub const END_LIST_COMMAND: &str = "323";

static mut CHANNELS: Vec<String> = vec![];

pub fn to_controller_message(message: Message) -> ControllerMessage {
    // commands with no ControllerMessage
    if &message.get_command()[..] == LIST_COMMAND {
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
        END_LIST_COMMAND => unsafe {
            ControllerMessage::ReceiveListChannels {
                channels: CHANNELS.clone(),
            }
        },
        _ => ControllerMessage::RegularMessage {
            message: message.to_string(),
        },
    }
}
