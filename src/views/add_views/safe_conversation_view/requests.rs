use crate::controller::controller_message::ControllerMessage::{
    self, CloseSafeView, SendSafeMessage,
};
use crate::views::ERROR_TEXT;
use gtk4::glib::Sender;

/// Sends a safe message.  
///
/// Receives a channel name, a member and the sender.
pub fn send_safe_message_request(
    message: String,
    client: String,
    sender: Sender<ControllerMessage>,
) {
    sender
        .send(SendSafeMessage { client, message })
        .expect(ERROR_TEXT);
}

/// Sends a close safe view to the controller.  
///
/// Receives a client and the sender.
pub fn close_safe_view_request(client: String, sender: Sender<ControllerMessage>) {
    sender.send(CloseSafeView { client }).expect(ERROR_TEXT);
}
