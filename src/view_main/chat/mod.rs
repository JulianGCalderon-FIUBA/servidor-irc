pub(crate) mod widgets_creation;

use gtk4 as gtk;
use gtk::{
    Box,
    Orientation,
    prelude::*,
    ScrolledWindow,
};

use self::widgets_creation::create_send_button;

use super::{MainView};

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
        self.send_message = create_send_button(
            message_box,
            self.input.clone(),
            scrolled_window.clone()
        );
        message_sender_box.append(&self.send_message);

        chat.append(&scrolled_window);
        chat.append(&message_sender_box);
        chat
    }
}