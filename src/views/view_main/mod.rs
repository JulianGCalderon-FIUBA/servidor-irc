mod chat;
mod conv_info;
mod sidebar;
pub mod widgets_creation;

use gtk::{
    glib::Sender, prelude::*, Application, ApplicationWindow, Button, Entry, Label, Orientation,
};
use gtk4 as gtk;

use self::chat::widgets_creation::create_message;
use self::widgets_creation::{create_button, create_separator};

use super::widgets_creation::{create_entry, create_main_box};

pub struct MainView {
    pub channels: Vec<Button>,
    pub add_channel: Button,
    pub clients: Vec<Button>,
    pub add_client: Button,
    pub messages: Vec<Label>,
    pub user_info: Button,
    pub send_message: Button,
    pub input: Entry,
    pub channel_info: Button,
    pub quit_channel: Button,
    pub func_channel: Button,
    sender: Sender<String>,
}

impl MainView {
    pub fn new(sender: Sender<String>) -> Self {
        Self {
            channels: vec![create_button("#channel1"), create_button("#channel2")],
            add_channel: create_button("+"),
            clients: vec![
                create_button("juli"),
                create_button("sol"),
                create_button("santi"),
                create_button("ana"),
            ],
            add_client: create_button("+"),
            messages: vec![create_message("hola!")],
            user_info: create_button("info"),
            input: create_entry("Message..."),
            send_message: create_button("send"),
            quit_channel: create_button("x"),
            channel_info: create_button("info"),
            func_channel: create_button("func"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(&app)
            .title("Lemon Pie IRC")
            .build();

        let main_box = create_main_box(Orientation::Horizontal, 800, 1200);
        main_box.add_css_class("main_box");

        let sidebar = self.create_sidebar();
        main_box.append(&sidebar);

        let separator = create_separator();
        main_box.append(&separator);

        let chat = self.create_chat();
        main_box.append(&chat);

        let conv_info = self.create_conv_info();
        main_box.append(&conv_info);

        window.set_child(Some(&main_box));

        window
    }
}
