use gtk::glib::Sender;
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

/// Sends a change conversation request to the controller.  
/// 
/// Receives a conversation.
pub fn change_conversation_request(conversation: String, sender: Sender<ControllerMessage>) {
    let request = ControllerMessage::ChangeConversation {
        nickname: conversation,
    };
    sender.send(request).expect("ERROR: change conversation");
}

/// Sends a quit request to the controller.  
/// 
/// Receives nothing.
pub fn quit_request(sender: Sender<ControllerMessage>) {
    let request = ControllerMessage::Quit {};
    sender.send(request).expect("ERROR: quit")
}
