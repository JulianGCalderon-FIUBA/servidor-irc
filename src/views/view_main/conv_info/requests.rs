use gtk::glib::Sender;
use gtk4 as gtk;

use crate::{ controller::controller_message::ControllerMessage, views::ERROR_TEXT };

pub fn quit_channel_request(sender: Sender<ControllerMessage>) {
    sender.send(ControllerMessage::QuitChannel {}).expect(ERROR_TEXT);
}

pub fn change_conversation_request(sender: Sender<ControllerMessage>) {
    let change_conv = ControllerMessage::ChangeConversation {
        nickname: "".to_string(),
    };
    sender.send(change_conv).expect(ERROR_TEXT);
}

pub fn add_invite_view_request(sender: Sender<ControllerMessage>) {
    sender.send(ControllerMessage::AddInviteView {}).expect(ERROR_TEXT);
}