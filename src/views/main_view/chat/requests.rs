use crate::views::main_view::ControllerMessage::SendPrivMessage;
use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};
use gtk4::glib::Sender;

/// Sends a private message request to the controller.  
///
/// Receives the message.
pub fn priv_message_request(input_text: String, sender: Sender<ControllerMessage>) {
    let priv_message = SendPrivMessage {
        message: input_text,
    };
    sender.send(priv_message).expect(ERROR_TEXT);
}
