use crate::controller::controller_message::ControllerMessage::{self, SendJoinMessage};
use crate::views::ERROR_TEXT;
use gtk4::glib::Sender;

/// Sends a join request to the controller.  
///
/// Receives a channel name and the sender.
pub fn join_channel_request(input: String, sender: Sender<ControllerMessage>) {
    sender
        .send(SendJoinMessage { channel: input })
        .expect(ERROR_TEXT);
}
