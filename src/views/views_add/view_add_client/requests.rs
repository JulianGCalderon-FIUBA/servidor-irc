use gtk4::glib::{GString, Sender};

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

pub fn add_client_button_request(client: GString, sender: Sender<ControllerMessage>) {
    let add_client = ControllerMessage::AddNewClient { client };
    sender.send(add_client).expect(ERROR_TEXT);
}
