use gtk::glib::{GString, Sender};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

pub fn change_conversation_request(conversation: GString, sender: Sender<ControllerMessage>) {
    let request = ControllerMessage::ChangeConversation {
        nickname: conversation.to_string(),
    };
    sender.send(request).expect("ERROR: change conversation");
}