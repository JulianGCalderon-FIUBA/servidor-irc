use gtk4::glib::Sender;

use crate::{
    controller::controller_message::ControllerMessage::{self, SendInviteMessage},
    views::ERROR_TEXT,
};

/// Sends a invite request to the controller.  
///
/// Receives a channel name and the sender.
pub fn invite_request(channel: String, sender: Sender<ControllerMessage>) {
    let invite = SendInviteMessage { channel };
    sender.send(invite).expect(ERROR_TEXT);
}
