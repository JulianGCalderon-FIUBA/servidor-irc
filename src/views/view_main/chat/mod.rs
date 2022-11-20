pub mod widgets_creation;
pub mod requests;

use gtk::{ glib::Sender, prelude::*, Box, Entry };
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::view_main::utils::entry_is_valid,
};

use self::{widgets_creation::{ create_chat_box, create_message_sender_box, create_received_message, create_send_message }, requests::priv_message_request};

use super::{ utils::adjust_scrollbar, MainView };

const RECEIVED_MESSAGE_CSS: &str = "received_message";
const SEND_MESSAGE_CSS: &str = "send_message";
const CHAT_CSS: &str = "chat";
const MESSAGE_BOX_CSS: &str = "message_box";


impl MainView {
    pub fn create_chat(&mut self) -> Box {
        let chat = create_chat_box();
        let message_sender_box = create_message_sender_box();

        self.user_info.connect_clicked(|_| println!("Hi"));
        message_sender_box.append(&self.user_info);

        self.input.set_hexpand(true);
        self.input.set_width_request(500);
        message_sender_box.append(&self.input);

        self.scrollwindow_chat.set_child(Some(&self.message_box));

        self.connect_send_button(self.input.clone(), self.sender.clone());
        message_sender_box.append(&self.send_message);

        chat.append(&self.current_chat);
        chat.append(&self.scrollwindow_chat);
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


    pub fn receive_priv_message(&mut self, message: String, _nickname: String) {
        let message = create_received_message(&message);
        self.message_box.append(&message);
        adjust_scrollbar(self.scrollwindow_chat.clone());
        self.messages.push(message);
    }

    pub fn send_message(&mut self, message: String) {
        let message = create_send_message(&message);
        self.message_box.append(&message);
        adjust_scrollbar(self.scrollwindow_chat.clone());
        self.messages.push(message);
    }
}