use gtk4::glib::Sender;

use crate::views::ip_view::ControllerMessage::OpenRegisterView;
use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a to register request to the controller.  
///
/// Receives an address and the sender.
pub fn to_register_request(address: String, sender: Sender<ControllerMessage>) {
    let to_register = OpenRegisterView { address };
    sender.send(to_register).expect(ERROR_TEXT);
}
