mod chat;
mod conv_info;
mod sidebar;
pub mod widgets_creation;
pub mod utils;

use gtk::{
    glib::{ GString, Sender },
    prelude::*,
    Application,
    ApplicationWindow,
    Box,
    Button,
    Entry,
    Label,
    Orientation,
};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::widgets_creation::{
    create_button,
    create_current_chat,
    create_message_box,
    create_separator,
    create_channels_box,
    create_clients_box, create_add_button,
};

use super::widgets_creation::{ create_entry, create_main_box };

pub struct MainView {
    pub channels_box: Box,
    pub add_channel: Button,
    pub clients_box: Box,
    pub add_client: Button,
    pub current_chat: Label,
    pub message_box: Box,
    pub user_info: Button,
    pub send_message: Button,
    pub input: Entry,
    pub channel_info: Button,
    pub quit_channel: Button,
    pub func_channel: Button,
    sender: Sender<ControllerMessage>,
}

impl MainView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            channels_box: create_channels_box(),
            add_channel: create_add_button("+"),
            clients_box: create_clients_box(),
            add_client: create_add_button("+"),
            current_chat: create_current_chat(""),
            message_box: create_message_box(),
            user_info: create_button("info"),
            input: create_entry("Message..."),
            send_message: create_button("send"),
            quit_channel: create_button("x"),
            channel_info: create_button("info"),
            func_channel: create_button("func"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application, nickname: GString) -> ApplicationWindow {
        self.user_info.set_label(&nickname);

        let window = ApplicationWindow::builder().application(&app).title("Lemon Pie IRC").build();

        let main_box = create_main_box(Orientation::Horizontal, 800, 600);
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