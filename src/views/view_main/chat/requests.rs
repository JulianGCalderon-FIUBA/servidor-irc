use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};
use gtk::glib::{GString, Sender};
use gtk4 as gtk;

/// Sends a private message request to the controller.  
/// 
/// Receives the message.
pub fn priv_message_request(input_text: GString, sender: Sender<ControllerMessage>) {
    let priv_message = ControllerMessage::SendPrivMessage {
        message: input_text,
    };
    sender.send(priv_message).expect(ERROR_TEXT);
}
