use gtk::glib::Sender;
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a to register request to the controller.  
///
/// Receives an address.
pub fn accept_request(client: String, address: String, sender: Sender<ControllerMessage>) {
    let accept_dcc_chat = ControllerMessage::AcceptDccChat { client, address };
    sender.send(accept_dcc_chat).expect(ERROR_TEXT);
}
