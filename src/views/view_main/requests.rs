use gtk::glib::Sender;
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

pub fn change_conversation_request(conversation: String, sender: Sender<ControllerMessage>) {
    let request = ControllerMessage::ChangeConversation {
        nickname: conversation,
    };
    sender.send(request).expect("ERROR: change conversation");
}
