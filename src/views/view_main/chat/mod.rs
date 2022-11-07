pub mod widgets_creation;

use gtk::{glib::Sender, prelude::*, Box, Entry, Orientation, ScrolledWindow};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::widgets_creation::{create_message, create_received_message};

use super::MainView;

impl MainView {
    pub fn create_chat(&mut self) -> Box {
        let chat = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::End)
            .hexpand(true)
            .build();
        chat.add_css_class("chat");

        let message_sender_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        self.user_info.connect_clicked(|_| println!("Hi"));
        message_sender_box.append(&self.user_info);

        self.input.set_hexpand(true);
        self.input.set_width_request(500);
        message_sender_box.append(&self.input);

        let scrolled_window: ScrolledWindow = ScrolledWindow::builder()
            .min_content_height(600)
            .max_content_width(500)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .margin_bottom(20)
            .child(&self.message_box)
            .build();

        scrolled_window.add_css_class("message_box");

        self.connect_send_button(
            self.message_box.clone(),
            self.input.clone(),
            scrolled_window.clone(),
            self.sender.clone(),
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
        sender: Sender<ControllerMessage>,
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
            sender
                .send(priv_message)
                .expect("Error: private message command");

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

fn entry_is_valid(entry_text: &str) -> bool {
    !entry_text.is_empty()
}
