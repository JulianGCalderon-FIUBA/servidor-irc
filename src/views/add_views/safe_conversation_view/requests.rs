use crate::controller::controller_message::ControllerMessage::{self, SendSafeMessage};
use crate::views::ERROR_TEXT;
use gtk::glib::Sender;

use gtk4 as gtk;

/// Sends a safe message.  
///
/// Receives a channel name and a member.
pub fn send_safe_message_request(message: String, client: String, sender: Sender<ControllerMessage>) {
    sender
        .send(SendSafeMessage { client, message })
        .expect(ERROR_TEXT);
}
