use gtk4::glib::Sender;

use crate::{
    controller::controller_message::ControllerMessage::{self, Register},
    views::ERROR_TEXT,
};

/// Sends a register request to the controller.  
///
/// Receives a password, nickname, username, a realname and the sender.
pub fn register_request(
    pass: String,
    nickname: String,
    username: String,
    realname: String,
    sender: Sender<ControllerMessage>,
) {
    let register = Register {
        pass,
        nickname,
        username,
        realname,
    };
    sender.send(register).expect(ERROR_TEXT);
}
