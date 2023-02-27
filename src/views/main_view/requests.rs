use gtk4::glib::Sender;

use crate::views::main_view::ControllerMessage::{ChangeConversation, SendQuitMessage};
use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a change conversation request to the controller.  
///
/// Receives a conversation and the sender.
pub fn change_conversation_request(conversation: String, sender: Sender<ControllerMessage>) {
    let request = ChangeConversation {
        nickname: conversation,
    };
    sender.send(request).expect(ERROR_TEXT);
}

/// Sends a quit request to the controller.  
///
/// Receives the sender.
pub fn quit_request(sender: Sender<ControllerMessage>) {
    let request = SendQuitMessage {};
    sender.send(request).expect(ERROR_TEXT)
}
