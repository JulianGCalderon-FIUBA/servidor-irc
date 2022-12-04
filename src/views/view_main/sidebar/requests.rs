use gtk::glib::Sender;
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

pub fn add_view_to_add_client_request(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::SendNamesMessageToAddClient{})
        .expect(ERROR_TEXT);
}

pub fn send_list_request(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::SendListMessage {})
        .expect(ERROR_TEXT);
}
