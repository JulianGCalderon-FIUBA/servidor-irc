use gtk::glib::{GString, Sender};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

pub fn register_request(
    pass: GString,
    nickname: GString,
    username: GString,
    realname: GString,
    address: GString,
    sender: Sender<ControllerMessage>,
) {
    let register = ControllerMessage::Register {
        pass,
        nickname,
        username,
        realname,
        address,
    };
    sender.send(register).expect("Error: pass command");
}

pub fn change_view_to_main_request(nickname: GString, sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::ChangeViewToMain { nickname })
        .expect("Error: pass command");
}
