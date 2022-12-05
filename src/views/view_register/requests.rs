use gtk::glib::{GString, Sender};
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a register request to the controller.  
/// 
/// Receives a password, nickname, username and a realname
pub fn register_request(
    pass: GString,
    nickname: GString,
    username: GString,
    realname: GString,
    sender: Sender<ControllerMessage>,
) {
    let register = ControllerMessage::Register {
        pass,
        nickname,
        username,
        realname,
    };
    sender.send(register).expect(ERROR_TEXT);
}
