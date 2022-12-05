use gtk4::glib::{GString, Sender};

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a add client request to the controller.  
///
/// Receives a client name.
pub fn add_client_button_request(new_client: GString, sender: Sender<ControllerMessage>) {
    let add_client = ControllerMessage::AddNewClient { new_client };
    sender.send(add_client).expect(ERROR_TEXT);
}
