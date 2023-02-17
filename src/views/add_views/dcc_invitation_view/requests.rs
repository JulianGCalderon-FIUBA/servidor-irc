use gtk::glib::Sender;
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a accept request to the controller.  
///
/// Receives a cleint and an address.
pub fn accept_request(client: String, address: String, sender: Sender<ControllerMessage>) {
    let accept_dcc_chat = ControllerMessage::AcceptDccChat { client, address };
    sender.send(accept_dcc_chat).expect(ERROR_TEXT);
}

/// Sends a decline request to the controller.  
///
/// Receives a client.
pub fn decline_request(client: String, sender: Sender<ControllerMessage>) {
    let decline_dcc_chat = ControllerMessage::DeclineDccChat { client };
    sender.send(decline_dcc_chat).expect(ERROR_TEXT);
}
