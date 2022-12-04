use crate::controller::controller_message::ControllerMessage;
use crate::views::ERROR_TEXT;
use gtk::glib::Sender;

use gtk4 as gtk;

pub fn kick_request(member: String, sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::KickMember { channel: todo!(), member })
        .expect(ERROR_TEXT);
}