use crate::message::Message;

use super::controller_message::ControllerMessage;

pub const PRIVMSG_COMMAND: &str = "PRIVMSG";
// pub const JOIN_COMMAND: &str = "JOIN";

pub fn to_controller_message(message: Message) -> ControllerMessage {
    match &message.get_command()[..] {
        PRIVMSG_COMMAND => ControllerMessage::ReceivePrivMessage {
            nickname: message.get_prefix().clone().unwrap(),
            message: message.get_trailing().clone().unwrap(),
        },
        // JOIN_COMMAND=> ControllerMessage::AddNewChannel {
        //     channel: message.get_prefix().clone().unwrap()
        // },
        _ => ControllerMessage::RegularMessage {
            message: message.to_string(),
        },
    }
}
