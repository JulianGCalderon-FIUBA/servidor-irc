pub mod widgets_creation;

use gtk::{ glib::Sender, prelude::*, Box, Entry, Orientation, ScrolledWindow };
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::view_main::utils::entry_is_valid,
};

use self::widgets_creation::{ create_message, create_received_message, create_chat_box, create_message_sender_box, create_scrollwindow_chat };

use super::MainView;


impl MainView {
    pub fn create_chat(&mut self) -> Box {
        let chat = create_chat_box();
        let message_sender_box = create_message_sender_box();

        self.user_info.connect_clicked(|_| println!("Hi"));
        message_sender_box.append(&self.user_info);

        self.input.set_hexpand(true);
        self.input.set_width_request(500);
        message_sender_box.append(&self.input);

        let scrolled_window: ScrolledWindow = create_scrollwindow_chat(&self.message_box);

        self.connect_send_button(
            self.message_box.clone(),
            self.input.clone(),
            scrolled_window.clone(),
            self.sender.clone()
        );
        message_sender_box.append(&self.send_message);

        chat.append(&self.current_chat);
        chat.append(&scrolled_window);
        chat.append(&message_sender_box);
        chat
    }

    fn connect_send_button(
        &self,
        message_box: Box,
        input: Entry,
        scrolled_window: ScrolledWindow,
        sender: Sender<ControllerMessage>
    ) {
        let nickname_receiver = self.current_chat.label().to_string();

        self.send_message.connect_clicked(move |_| {
            let input_text = input.text();
            if !entry_is_valid(&input_text) {
                return;
            }

            println!("Send message to: {}", nickname_receiver);

            let priv_message = ControllerMessage::SendPrivMessage {
                message: input_text.clone(),
            };
            sender.send(priv_message).expect("Error: private message command");

            let message = create_message(&input_text);
            message.add_css_class("message");
            message_box.append(&message);

            let adj = scrolled_window.vadjustment();
            adj.set_upper(adj.upper() + adj.page_size());
            adj.set_value(adj.upper());
            scrolled_window.set_vadjustment(Some(&adj));

            input.set_text("");
        });
    }

    pub fn receive_priv_message(&self, message: String, _nickname: String) {
        let message = create_received_message(&message);
        message.add_css_class("message");
        self.message_box.append(&message);
    }
}