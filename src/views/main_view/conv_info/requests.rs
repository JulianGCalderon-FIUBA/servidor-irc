use gtk4::glib::Sender;

use crate::{
    controller::controller_message::ControllerMessage::{
        self, RemoveConversation, SendNamesMessageToInviteClient, SendNamesMessageToKnowMembers,
        SendPartMessage, SendSafeConversationRequest,
    },
    views::ERROR_TEXT,
};

/// Sends a quit channel request to the controller.
///
/// Receives the sender.
pub fn quit_channel_request(sender: Sender<ControllerMessage>) {
    sender.send(SendPartMessage {}).expect(ERROR_TEXT);
}

/// Sends a remove conversation request to the controller.
///
/// Receives the sender.
pub fn remove_conversation_request(sender: Sender<ControllerMessage>) {
    sender.send(RemoveConversation {}).expect(ERROR_TEXT);
}

/// Sends an add invite view request to the controller.
///
/// Receives the sender.
pub fn add_invite_view_request(sender: Sender<ControllerMessage>) {
    sender
        .send(SendNamesMessageToInviteClient {})
        .expect(ERROR_TEXT);
}

/// Sends an add safe conversation view request to the controller.
///
/// Receives the sender.
pub fn add_safe_conversation_view_request(sender: Sender<ControllerMessage>) {
    sender
        .send(SendSafeConversationRequest {})
        .expect(ERROR_TEXT);
}

/// Sends a names request to the controller.
///
/// Receives the sender.
pub fn send_names_request(sender: Sender<ControllerMessage>) {
    sender
        .send(SendNamesMessageToKnowMembers {})
        .expect(ERROR_TEXT);
}