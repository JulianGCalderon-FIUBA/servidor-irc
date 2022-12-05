mod chat;
mod conv_info;
pub mod requests;
mod sidebar;
pub mod utils;
pub mod widgets_creation;

use std::collections::HashMap;

use gtk::{
    glib::Sender,
    prelude::*,
    Application, ApplicationWindow, Box, Button, Entry, Label,
    Orientation::{Horizontal, Vertical},
    ScrolledWindow,
};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::{
    chat::widgets_creation::create_scrollwindow_chat,
    requests::quit_request,
    widgets_creation::{
        create_add_button, create_channels_and_client_box, create_current_chat, create_message_box,
        create_notifications_button, create_scrollwindow, create_welcome_box,
    },
};

use super::{
    widgets_creation::{
        build_application_window, create_button_with_margin, create_entry, create_error_label,
        create_separator,
    },
    MAIN_BOX_CSS,
};

const ADD_BUTTON_TEXT: &str = "+";
const ENTRY_PLACEHOLDER: &str = "Message...";
const SEND_BUTTON_TEXT: &str = "Send";
const QUIT_BUTTON_TEXT: &str = "x";
const MEMBERS_BUTTON_TEXT: &str = "Members";
const INVITE_BUTTON_TEXT: &str = "Invite to channel";
const ADD_BUTTON_CSS: &str = "add";
const CURRENT_CHAT_TITLE_CSS: &str = "current_chat";
const WELCOME_MESSAGE: &str = "Open a new chat to start...";
const WELCOME_MESSAGE_CSS: &str = "welcome_message";
const WELCOME_TITLE: &str = "Welcome to Lemon Pie IRC!";
const WELCOME_TITLE_CSS: &str = "welcome_title";
const NO_NOTIFICATIONS_TEXT: &str = "🔔 notifications (0)";
const NO_NOTIFICATIONS_CSS: &str = "notifications_button";
const DISABLE_BUTTON_CSS: &str = "disabled_button";
pub struct MainView {
    pub channels_box: Box,
    pub channels_buttons: Vec<Button>,
    pub scrollwindow_channels: ScrolledWindow,
    pub add_channel: Button,
    pub clients_box: Box,
    pub clients_buttons: Vec<Button>,
    pub scrollwindow_clients: ScrolledWindow,
    pub add_client: Button,
    pub notifications_button: Button,
    pub notifications: Vec<String>,
    pub welcome_box: Box,
    pub current_chat: Label,
    scrollwindow_chat: ScrolledWindow,
    pub message_box: Box,
    pub messages: HashMap<String, Vec<Vec<Label>>>,
    pub user_info: Button,
    pub send_message: Button,
    pub input: Entry,
    pub error_label: Label,
    pub channel_members_button: Button,
    pub quit_channel_button: Button,
    pub invite_button: Button,
    sender: Sender<ControllerMessage>,
}

impl MainView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            channels_box: create_channels_and_client_box(),
            channels_buttons: vec![],
            scrollwindow_channels: create_scrollwindow(),
            add_channel: create_add_button(ADD_BUTTON_TEXT),
            clients_box: create_channels_and_client_box(),
            clients_buttons: vec![],
            scrollwindow_clients: create_scrollwindow(),
            add_client: create_add_button(ADD_BUTTON_TEXT),
            notifications_button: create_notifications_button(),
            notifications: vec![],
            welcome_box: create_welcome_box(),
            current_chat: create_current_chat(""),
            scrollwindow_chat: create_scrollwindow_chat(),
            message_box: create_message_box(),
            messages: HashMap::new(),
            user_info: create_button_with_margin(""),
            input: create_entry(ENTRY_PLACEHOLDER),
            send_message: create_button_with_margin(SEND_BUTTON_TEXT),
            error_label: create_error_label(),
            quit_channel_button: create_button_with_margin(QUIT_BUTTON_TEXT),
            channel_members_button: create_button_with_margin(MEMBERS_BUTTON_TEXT),
            invite_button: create_button_with_margin(INVITE_BUTTON_TEXT),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application, nickname: String) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = Box::builder()
            .orientation(Horizontal)
            .halign(gtk::Align::Center)
            .width_request(600)
            .build();
        main_box.add_css_class(MAIN_BOX_CSS);

        let sidebar = self.create_sidebar();
        main_box.append(&sidebar);

        let separator = create_separator(Vertical);
        main_box.append(&separator);

        let chat = self.create_chat();
        main_box.append(&chat);

        let conv_info = self.create_conv_info(&nickname);
        main_box.append(&conv_info);

        window.set_child(Some(&main_box));

        Self::close_view(window.clone(), app, self.sender.clone());

        window
    }

    fn close_view(window: ApplicationWindow, app: Application, sender: Sender<ControllerMessage>) {
        window.connect_destroy(move |_| {
            quit_request(sender.clone());
            app.quit();
        });
    }
}
