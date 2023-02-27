use gtk4::glib::Sender;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a close dcc chat to the controller.  
///
/// Receives a client and the sender.
pub fn close_safe_conv_request(client: String, sender: Sender<ControllerMessage>) {
    let close_dcc = ControllerMessage::CloseDccChat { client };
    sender.send(close_dcc).expect(ERROR_TEXT);
}
