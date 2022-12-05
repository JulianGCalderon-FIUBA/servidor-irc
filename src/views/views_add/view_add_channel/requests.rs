use crate::controller::controller_message::ControllerMessage;
use crate::views::ERROR_TEXT;
use gtk::glib::Sender;

use gtk4 as gtk;

pub fn join_channel_request(input: String, sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::JoinChannel { channel: input })
        .expect(ERROR_TEXT);
}
