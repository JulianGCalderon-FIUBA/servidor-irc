use gtk::glib::Sender;
use crate::controller::controller_message::ControllerMessage;
use crate::views::ERROR_TEXT;

use gtk4 as gtk;
use gtk4::glib::GString;

pub fn join_channel_request(input: GString, sender: Sender<ControllerMessage>) {
    sender.send(ControllerMessage::JoinChannel { channel: input }).expect(ERROR_TEXT);
}