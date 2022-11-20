use gtk4::glib::{GString, Sender};

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

pub fn invite_request(channel: GString, sender: Sender<ControllerMessage>) {
    let invite = ControllerMessage::SendInviteMessage { channel };
    sender.send(invite).expect(ERROR_TEXT);
}
