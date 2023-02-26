use crate::controller::controller_message::ControllerMessage::{self, SendKickMessage};
use crate::views::ERROR_TEXT;
use gtk4::glib::Sender;

/// Sends a kick request to the controller.  
///
/// Receives a channel name and a member.
pub fn kick_request(channel: String, member: String, sender: Sender<ControllerMessage>) {
    sender
        .send(SendKickMessage { channel, member })
        .expect(ERROR_TEXT);
}
