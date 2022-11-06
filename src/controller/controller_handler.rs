use crate::message::Message;

use super::controller_message::ControllerMessage;

pub fn to_controller_message(message: Message) -> ControllerMessage {
    ControllerMessage::RegularMessage { message: message.to_string() }
}