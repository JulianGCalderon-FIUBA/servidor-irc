pub mod requests;
pub mod widgets_creation;

use gtk::{
    glib::Sender,
    prelude::*,
    Box, Entry,
};
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::{view_main::utils::entry_is_valid, widgets_creation::create_label},
};

use self::{
    requests::priv_message_request,
    widgets_creation::{
        create_chat_box, create_message_sender_box, create_received_message, create_send_message,
        create_sender_nickname_label,
    },
};

use super::{utils::adjust_scrollbar, MainView};

const RECEIVED_MESSAGE_CSS: &str = "received_message";
const SEND_MESSAGE_CSS: &str = "send_message";
const CHAT_CSS: &str = "chat";
const MESSAGE_BOX_CSS: &str = "message_box";

impl MainView {
    pub fn create_chat(&mut self) -> Box {
        // self.current_chat.set_label(nickname);

        let chat = create_chat_box();
        let message_sender_box = create_message_sender_box();

        self.user_info.connect_clicked(|_| println!("Hi"));
        message_sender_box.append(&self.user_info);

        self.input.set_hexpand(true);
        self.input.set_width_request(500);
        message_sender_box.append(&self.input);

        self.scrollwindow_chat.set_child(Some(&self.message_box));
        self.scrollwindow_chat.set_visible(false);

        self.connect_send_button(self.input.clone(), self.sender.clone());
        message_sender_box.append(&self.send_message);
        self.send_message.set_sensitive(false);
        

        chat.append(&self.current_chat);
        chat.append(&self.scrollwindow_chat);
        chat.append(&self.welcome_box);
        chat.append(&message_sender_box);
        chat
    }

    fn connect_send_button(&self, input: Entry, sender: Sender<ControllerMessage>) {
        self.send_message.connect_clicked(move |_| {
            let input_text = input.text();
            if !entry_is_valid(&input_text) {
                return;
            }

            priv_message_request(input_text, sender.clone());

            input.set_text("");
        });
    }

    pub fn receive_priv_channel_message(
        &mut self,
        message: String,
        sender_nickname: String,
        channel: String,
        current_conv: String,
    ) {
        if sender_nickname == self.user_info.label().unwrap() || channel != current_conv {
            return;
        }

        let sender_nickname_label = create_sender_nickname_label(&sender_nickname);
        if Self::should_show_nickname(self.messages.get(&channel), sender_nickname) {
            self.message_box.append(&sender_nickname_label);
        }

        let message = create_received_message(&message);
        self.message_box.append(&message);
        adjust_scrollbar(self.scrollwindow_chat.clone());

        self.messages
            .get_mut(&channel)
            .unwrap()
            .push(vec![message, sender_nickname_label]);
    }

    pub fn receive_priv_client_message(
        &mut self,
        message: String,
        nickname: String,
        current_conv: String,
    ) {
        if nickname != current_conv {
            return;
        }
        let message = create_received_message(&message);
        self.message_box.append(&message);
        adjust_scrollbar(self.scrollwindow_chat.clone());

        self.messages
            .get_mut(&nickname)
            .unwrap()
            .push(vec![message, create_sender_nickname_label("")]);
    }

    pub fn send_message(&mut self, message: String, nickname: String) {
        let message = create_send_message(&message);
        self.message_box.append(&message);
        adjust_scrollbar(self.scrollwindow_chat.clone());

        self.messages
            .get_mut(&nickname)
            .unwrap()
            .push(vec![message, create_label("")]);
    }

    pub fn should_show_nickname(
        messages: Option<&Vec<Vec<gtk4::Label>>>,
        sender_nickname: String,
    ) -> bool {
        Self::prev_message_has_different_sender(messages, sender_nickname)
            || messages.unwrap().is_empty()
    }

    pub fn prev_message_has_different_sender(
        messages: Option<&Vec<Vec<gtk4::Label>>>,
        sender_nickname: String,
    ) -> bool {
        messages.is_some()
            && messages.unwrap().last().is_some()
            && messages.unwrap().last().unwrap()[1].text() != sender_nickname
    }
}
