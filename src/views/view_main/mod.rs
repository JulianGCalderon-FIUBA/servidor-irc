mod chat;
mod conv_info;
mod sidebar;
pub mod utils;
pub mod widgets_creation;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Application, ApplicationWindow, Box, Button, Entry, Label,
    Orientation::{Horizontal, Vertical},
    ScrolledWindow,
};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::{
    chat::widgets_creation::create_scrollwindow_chat,
    widgets_creation::{
        create_add_button, create_channels_and_client_box, create_current_chat, create_message_box,
        create_scrollwindow_sidebar,
    },
};

use super::{
    widgets_creation::{
        build_application_window, create_button_with_margin, create_entry, create_main_box,
        create_separator,
    },
    MAIN_BOX_CSS,
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

const ADD_BUTTON_TEXT: &str = "+";
const ENTRY_PLACEHOLDER: &str = "Message...";
const SEND_BUTTON_TEXT: &str = "Send";
const QUIT_BUTTON_TEXT: &str = "x";
const INFO_BUTTON_TEXT: &str = "Info";
const FUNC_BUTTON_TEXT: &str = "Func";
const ADD_BUTTON_CSS: &str = "add";
const CURRENT_CHAT_TITLE_CSS: &str = "current_chat";

impl MainView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            channels_box: create_channels_and_client_box(),
            channels_button: vec![],
            scrollwindow_channels: create_scrollwindow_sidebar(),
            add_channel: create_add_button(ADD_BUTTON_TEXT),
            clients_box: create_channels_and_client_box(),
            scrollwindow_clients: create_scrollwindow_sidebar(),
            add_client: create_add_button(ADD_BUTTON_TEXT),
            current_chat: create_current_chat(""),
            scrollwindow_chat: create_scrollwindow_chat(),
            message_box: create_message_box(),
            messages: vec![],
            user_info: create_button_with_margin(""),
            input: create_entry(ENTRY_PLACEHOLDER),
            send_message: create_button_with_margin(SEND_BUTTON_TEXT),
            quit_channel: create_button_with_margin(QUIT_BUTTON_TEXT),
            channel_info: create_button_with_margin(INFO_BUTTON_TEXT),
            func_channel: create_button_with_margin(FUNC_BUTTON_TEXT),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application, nickname: GString) -> ApplicationWindow {
        self.user_info.set_label(&nickname);

        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box(Horizontal, 800, 600);
        main_box.add_css_class(MAIN_BOX_CSS);

        let sidebar = self.create_sidebar();
        main_box.append(&sidebar);

        let separator = create_separator(Vertical);
        main_box.append(&separator);

        let chat = self.create_chat();
        main_box.append(&chat);

        let conv_info = self.create_conv_info();
        main_box.append(&conv_info);

        window.set_child(Some(&main_box));

        window
    }
}
