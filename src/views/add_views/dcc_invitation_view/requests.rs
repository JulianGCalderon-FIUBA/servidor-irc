use gtk4::glib::Sender;

use std::net::SocketAddr;

use crate::views::add_views::dcc_invitation_view::ControllerMessage::{
    AcceptDccChat, DeclineDccChat,
};
use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a accept request to the controller.  
///
/// Receives a cleint and an address.
pub fn accept_request(client: String, address: SocketAddr, sender: Sender<ControllerMessage>) {
    let accept_dcc_chat = AcceptDccChat { client, address };
    sender.send(accept_dcc_chat).expect(ERROR_TEXT);
}

/// Sends a decline request to the controller.  
///
/// Receives a client.
pub fn decline_request(client: String, sender: Sender<ControllerMessage>) {
    let decline_dcc_chat = DeclineDccChat { client };
    sender.send(decline_dcc_chat).expect(ERROR_TEXT);
}
