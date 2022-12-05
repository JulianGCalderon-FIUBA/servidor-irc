use gtk::glib::{GString, Sender};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

pub fn to_register_request(address: String, sender: Sender<ControllerMessage>) {
    let to_register = ControllerMessage::ToRegister { address };
    sender.send(to_register).expect("Error: pass command");
}

pub fn change_view_to_main_request(nickname: GString, sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::ChangeViewToMain {
            nickname: nickname.to_string(),
        })
        .expect("Error: pass command");
}
