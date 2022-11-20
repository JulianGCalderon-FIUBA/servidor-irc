use crate::controller::controller_message::ControllerMessage;
use gtk::glib::{GString, Sender};
use gtk4 as gtk;

pub fn priv_message_request(input_text: GString, sender: Sender<ControllerMessage>) {
    let priv_message = ControllerMessage::SendPrivMessage {
        message: input_text,
    };
    sender
        .send(priv_message)
        .expect("Error: private message command");
}