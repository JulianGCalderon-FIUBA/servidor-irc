pub mod widgets_creation;

use gtk::{prelude::*, Box,Entry, Orientation, ScrolledWindow, glib::Sender};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::widgets_creation::create_message;

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

        let message_box = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_bottom(10)
            .halign(gtk::Align::Start)
            .build();

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
        self.input.set_width_request(600);
        message_sender_box.append(&self.input);

        let scrolled_window: ScrolledWindow = ScrolledWindow::builder()
            .min_content_height(800)
            .min_content_width(600)
            .margin_top(20)
            .margin_bottom(20)
            .child(&message_box)
            .build();

        scrolled_window.add_css_class("message_box");
    
        self.connect_send_button(message_box, self.input.clone(), scrolled_window.clone(), self.sender.clone());

        message_sender_box.append(&self.send_message);

        chat.append(&scrolled_window);
        chat.append(&message_sender_box);
        chat
    }

    fn connect_send_button(&self, message_box: Box, input: Entry, scrolled_window: ScrolledWindow, sender: Sender<ControllerMessage>) {
        self.send_message.connect_clicked(move |_| {
            let input_text = input.text();
            if !entry_is_valid(&input_text) {
                return;
            }
            
            let priv_message = ControllerMessage::SendPrivMessage { 
                nickname: "ana".to_string(), 
                message: input_text.clone() };
            sender.send(priv_message).expect("Error: private message command");
    
            let message = create_message(&input_text);
            message.add_css_class("message");
            message_box.append(&message);
    
            let adj = scrolled_window.vadjustment();
            adj.set_upper(adj.upper() + adj.page_size());
            adj.set_value(adj.upper());
            scrolled_window.set_vadjustment(Some(&adj));
        });
    }
}

fn entry_is_valid(entry_text: &str) -> bool {
    !entry_text.is_empty()
}