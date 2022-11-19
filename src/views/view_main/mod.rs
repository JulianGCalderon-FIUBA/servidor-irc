mod chat;
mod conv_info;
mod sidebar;
pub mod utils;
pub mod widgets_creation;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ScrolledWindow,
};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::{
    chat::widgets_creation::create_scrollwindow_chat,
    widgets_creation::{
        create_add_button, create_button, create_channels_box, create_clients_box,
        create_current_chat, create_message_box, create_scrollwindow_sidebar, create_separator,
    },
};

use super::{
    widgets_creation::{create_entry, create_main_box},
    APP_TITLE, MAIN_BOX_CSS,
};

pub struct MainView {
    pub channels_box: Box,
    pub channels_button: Vec<Button>,
    pub scrollwindow_channels: ScrolledWindow,
    pub add_channel: Button,
    pub clients_box: Box,
    pub scrollwindow_clients: ScrolledWindow,
    pub add_client: Button,
    pub current_chat: Label,
    scrollwindow_chat: ScrolledWindow,
    pub message_box: Box,
    pub messages: Vec<Label>,
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
            channels_button: vec![],
            scrollwindow_channels: create_scrollwindow_sidebar(),
            add_channel: create_add_button("+"),
            clients_box: create_clients_box(),
            scrollwindow_clients: create_scrollwindow_sidebar(),
            add_client: create_add_button("+"),
            current_chat: create_current_chat(""),
            scrollwindow_chat: create_scrollwindow_chat(),
            message_box: create_message_box(),
            messages: vec![],
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

        let window = ApplicationWindow::builder()
            .application(&app)
            .title(APP_TITLE)
            .build();

        let main_box = create_main_box(Orientation::Horizontal, 800, 600);
        main_box.add_css_class(MAIN_BOX_CSS);

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
