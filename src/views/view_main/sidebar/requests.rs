use gtk::glib::{GString, Sender};
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

pub fn change_conversation_request(conversation: GString, sender: Sender<ControllerMessage>) {
    let request = ControllerMessage::ChangeConversation {
        nickname: conversation.to_string(),
    };
    sender.send(request).expect("ERROR: change conversation");
}

pub fn add_view_to_add_client_request(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::AddViewToAddClient {})
        .expect(ERROR_TEXT);
}

pub fn send_list_request(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::SendListMessage {})
        .expect(ERROR_TEXT);
}
