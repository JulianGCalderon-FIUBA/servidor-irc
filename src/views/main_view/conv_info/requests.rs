use gtk::glib::Sender;
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage::{
        self, OpenSafeConversationView, RemoveConversation, SendNamesMessageToInviteClient,
        SendNamesMessageToKnowMembers, SendPartMessage,
    },
    views::ERROR_TEXT,
};

/// Sends a quit channel request to the controller.
///
/// Receives nothing.
pub fn quit_channel_request(sender: Sender<ControllerMessage>) {
    sender.send(SendPartMessage {}).expect(ERROR_TEXT);
}

/// Sends a remove conversation request to the controller.
///
/// Receives nothing.
pub fn remove_conversation_request(sender: Sender<ControllerMessage>) {
    sender.send(RemoveConversation {}).expect(ERROR_TEXT);
}

/// Sends an add invite view request to the controller.
///
/// Receives nothing.
pub fn add_invite_view_request(sender: Sender<ControllerMessage>) {
    sender
        .send(SendNamesMessageToInviteClient {})
        .expect(ERROR_TEXT);
}

pub fn add_safe_conversation_view_request(sender: Sender<ControllerMessage>) {
    sender.send(OpenSafeConversationView {}).expect(ERROR_TEXT);
}

/// Sends a names request to the controller.
///
/// Receives nothing.
pub fn send_names_request(sender: Sender<ControllerMessage>) {
    sender
        .send(SendNamesMessageToKnowMembers {})
        .expect(ERROR_TEXT);
}
