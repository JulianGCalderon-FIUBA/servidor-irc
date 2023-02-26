/// Contains definition of used requests.
pub mod requests;

/// Contains useful functions.
pub mod utils;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

mod chat;
mod conv_info;
mod sidebar;

use std::collections::HashMap;

use gtk4::{
    glib::Sender,
    prelude::ApplicationExt,
    traits::{BoxExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Box, Button, Entry, Label,
    Orientation::Vertical,
    ScrolledWindow,
};

use crate::controller::{
    controller_message::ControllerMessage,
    utils::{first_word_of_button, is_channel},
};

use self::{
    requests::quit_request,
    widgets_creation::{
        create_add_button, create_channels_and_client_box, create_current_chat,
        create_main_view_box, create_message_box, create_notifications_button, create_scrollwindow,
        create_welcome_box,
    },
};

use super::{
    widgets_creation::{
        build_application_window, create_button_with_margin, create_entry, create_error_label,
        create_scrollwindow_chat, create_separator,
    },
    ENTRY_PLACEHOLDER, SEND_BUTTON_TEXT,
};

const ADD_BUTTON_CSS: &str = "add";
const ADD_BUTTON_TEXT: &str = "+";
const CURRENT_CHAT_TITLE_CSS: &str = "current_chat";
const DISABLE_BUTTON_CSS: &str = "disabled_button";
const INVITE_BUTTON_TEXT: &str = "Invite to channel";
const MEMBERS_BUTTON_TEXT: &str = "Members";
const NO_NOTIFICATIONS_CSS: &str = "notifications_button";
const NO_NOTIFICATIONS_TEXT: &str = "🔔 notifications (0)";
const NOTIFICATION_ON_BUTTON_CSS: &str = "notifications_button_on";
const QUIT_BUTTON_TEXT: &str = "x";
const SAFE_CONVERSATION_BUTTON_TEXT: &str = "🔐 Safe conversation 🔐";
const SEND_FILE_BUTTON_TEXT: &str = "Send File";
const WELCOME_MESSAGE_CSS: &str = "welcome_message";
const WELCOME_MESSAGE: &str = "Open a new chat to start...";
const WELCOME_TITLE_CSS: &str = "welcome_title";
const WELCOME_TITLE: &str = "Welcome to Lemon Pie IRC!";
const CHAT_BUTTON_SELECTED_CSS: &str = "selected_chat_button";

/// Shows main view.
/// Contains the sidebar, chat and features.
/// Uses sender to communicate with controller.
pub struct MainView {
    add_channel: Button,
    add_client: Button,
    channel_members_button: Button,
    channels_box: Box,
    channels_buttons: Vec<Button>,
    clients_box: Box,
    clients_buttons: Vec<Button>,
    current_chat: Label,
    error_label: Label,
    input: Entry,
    invite_button: Button,
    message_box: Box,
    messages: HashMap<String, Vec<Vec<Label>>>,
    notifications_button: Button,
    notifications: Vec<String>,
    quit_channel_button: Button,
    safe_conversation_button: Button,
    send_file_button: Button,
    scrollwindow_channels: ScrolledWindow,
    scrollwindow_chat: ScrolledWindow,
    scrollwindow_clients: ScrolledWindow,
    send_message: Button,
    sender: Sender<ControllerMessage>,
    user_info: Button,
    welcome_box: Box,
}

impl MainView {
    /// Creates new [`MainView`]
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            add_channel: create_add_button(ADD_BUTTON_TEXT),
            add_client: create_add_button(ADD_BUTTON_TEXT),
            channel_members_button: create_button_with_margin(MEMBERS_BUTTON_TEXT),
            channels_box: create_channels_and_client_box(),
            channels_buttons: vec![],
            clients_box: create_channels_and_client_box(),
            clients_buttons: vec![],
            current_chat: create_current_chat(""),
            error_label: create_error_label(),
            input: create_entry(ENTRY_PLACEHOLDER),
            invite_button: create_button_with_margin(INVITE_BUTTON_TEXT),
            message_box: create_message_box(),
            messages: HashMap::new(),
            notifications_button: create_notifications_button(),
            notifications: vec![],
            quit_channel_button: create_button_with_margin(QUIT_BUTTON_TEXT),
            safe_conversation_button: create_button_with_margin(SAFE_CONVERSATION_BUTTON_TEXT),
            scrollwindow_channels: create_scrollwindow(),
            scrollwindow_chat: create_scrollwindow_chat(),
            scrollwindow_clients: create_scrollwindow(),
            send_message: create_button_with_margin(SEND_BUTTON_TEXT),
            sender,
            user_info: create_button_with_margin(""),
            welcome_box: create_welcome_box(),
            send_file_button: create_button_with_margin(SEND_FILE_BUTTON_TEXT),
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application, nickname: String) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_view_box();

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

    /// Closes the app.
    fn close_view(window: ApplicationWindow, app: Application, sender: Sender<ControllerMessage>) {
        window.connect_destroy(move |_| {
            quit_request(sender.clone());
            app.quit();
        });
    }

    pub fn find_button_by_name(&mut self, conv_name: &str) -> (Option<Button>, String) {
        let vector = if is_channel(conv_name) {
            self.channels_buttons.clone()
        } else {
            self.clients_buttons.clone()
        };
        let mut name = String::new();
        let button = vector.into_iter().find(|button| {
            name = first_word_of_button(button);
            name == conv_name
        });

        (button, name)
    }
}
