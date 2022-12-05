use gtk::glib::Sender;
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

pub fn to_register_request(address: String, sender: Sender<ControllerMessage>) {
    let to_register = ControllerMessage::ToRegister { address };
    sender.send(to_register).expect("Error: pass command");
}