pub mod requests;
mod widgets_creation;

use std::collections::{ hash_map::RandomState, HashMap };

use gtk::{
    glib::Sender,
    traits::{ BoxExt, ButtonExt, EditableExt, WidgetExt },
    Box,
    Button,
    Label,
    Orientation,
};
use gtk4 as gtk;

use crate::{
    controller::{ controller_message::ControllerMessage, utils::{ is_channel, is_not_empty } },
    ctcp::dcc_chat::{ self, DccChat },
    server::consts::channel::MAX_CHANNELS,
    views::{
        add_views::widgets_creation::create_title,
        main_view::{ ADD_BUTTON_CSS, DISABLE_BUTTON_CSS },
        widgets_creation::create_button_with_margin,
    },
};

use self::{
    requests::{
        add_notifications_view_request,
        add_user_info_view,
        add_view_to_add_client_request,
        send_list_request,
    },
    widgets_creation::create_separator_sidebar,
};

use super::{
    conv_info::{ DISABLE_SAFE_CONVERSATION_BUTTON_CSS, SAFE_CONVERSATION_BUTTON_CSS },
    requests::change_conversation_request,
    utils::{
        add_notification_to_button,
        adjust_scrollbar,
        deselect_conversation_button,
        remove_button_notifications_if_any,
        select_conversation_button,
    },
    MainView,
    NO_NOTIFICATIONS_TEXT,
};

const SAFE_BUTTON_TOOLTIP: &str =
    "Button is disabled because there is already an active safe conversation";
const CHANNELS_TITLE: &str = "Channels";
const CLIENTS_TITLE: &str = "Clients";

impl MainView {
    /// Creates sidebar widgets.
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder().width_request(200).orientation(Orientation::Vertical).build();

        //Channels box
        let channels_title = create_title(CHANNELS_TITLE);
        self.scrollwindow_channels.set_child(Some(&self.channels_box));
        self.connect_add_channel_button(self.add_channel.clone(), self.sender.clone());

        //Clients box
        let clients_title = create_title(CLIENTS_TITLE);
        self.scrollwindow_clients.set_child(Some(&self.clients_box));
        self.connect_add_client_button(self.add_client.clone(), self.sender.clone());

        self.connect_notifications_button(self.notifications_button.clone(), self.sender.clone());
        self.connect_user_info_button(self.user_info.clone(), self.sender.clone());

        sidebar.append(&channels_title);
        sidebar.append(&self.scrollwindow_channels);
        sidebar.append(&self.add_channel);
        sidebar.append(&create_separator_sidebar());
        sidebar.append(&clients_title);
        sidebar.append(&self.scrollwindow_clients);
        sidebar.append(&self.add_client);
        sidebar.append(&create_separator_sidebar());
        sidebar.append(&self.notifications_button);
        sidebar.append(&self.user_info);

        sidebar
    }

    /// Connects add client button.
    ///
    /// Sends add client request to the controller.
    fn connect_add_client_button(&self, button: Button, sender: Sender<ControllerMessage>) {
        button.connect_clicked(move |_| {
            add_view_to_add_client_request(sender.clone());
        });
    }

    fn connect_user_info_button(&self, button: Button, sender: Sender<ControllerMessage>) {
        button.connect_clicked(move |_| {
            add_user_info_view(sender.clone());
        });
    }

    /// Connects add channel button.
    ///
    /// Sends add channel request to the controller.
    pub fn connect_add_channel_button(&self, button: Button, sender: Sender<ControllerMessage>) {
        button.connect_clicked(move |_| {
            send_list_request(sender.clone());
        });
    }

    /// Connects notifications button.
    ///
    /// Sends add notification view request to the controller.
    pub fn connect_notifications_button(&self, button: Button, sender: Sender<ControllerMessage>) {
        let button_clone = button.clone();
        button.connect_clicked(move |_| {
            add_notifications_view_request(sender.clone());
            remove_button_notifications_if_any(&button_clone, NO_NOTIFICATIONS_TEXT);
        });
    }

    /// Adds channel to the sidebar.
    ///
    /// Creates new channel button.
    pub fn add_channel(&mut self, channel: String) {
        let channel_button = create_button_with_margin(&channel);
        self.connect_channel_client_button(
            channel_button.clone(),
            channel.clone(),
            self.sender.clone()
        );
        self.channels_box.append(&channel_button);
        self.channels_buttons.push(channel_button);
        if self.channels_buttons.len() >= MAX_CHANNELS {
            self.add_channel.remove_css_class(ADD_BUTTON_CSS);
            self.add_channel.add_css_class(DISABLE_BUTTON_CSS);
        }

        self.messages.insert(channel, vec![]);
        adjust_scrollbar(self.scrollwindow_channels.clone());
    }

    /// Adds client to the sidebar.
    ///
    /// Creates new client button.
    pub fn add_client(&mut self, client: &str) {
        let client_button = create_button_with_margin(client);
        self.connect_channel_client_button(
            client_button.clone(),
            client.to_string(),
            self.sender.clone()
        );
        self.clients_box.append(&client_button);
        self.clients_buttons.push(client_button);

        self.messages.insert(client.to_string(), vec![]);

        adjust_scrollbar(self.scrollwindow_clients.clone());
    }

    /// Connects conversation button.
    ///
    /// Sends change conversation request to the controller.
    pub fn connect_channel_client_button(
        &self,
        button: Button,
        channel_or_client: String,
        sender: Sender<ControllerMessage>
    ) {
        button.connect_clicked(move |_| {
            change_conversation_request(channel_or_client.clone(), sender.clone());
        });
    }

    /// Changes conversation view.
    ///
    /// Changes chat label and messages.
    pub fn change_conversation(
        &mut self,
        last_conv: String,
        conversation_label: String,
        dcc_chats: &HashMap<String, DccChat, RandomState>
    ) {
        self.update_chat_view_when_change_conversation(&conversation_label);
        self.update_last_chat_button_when_change_conversation(&last_conv);
        self.update_chat_button_when_clicked(&conversation_label);
        self.update_safe_conversation_button(&conversation_label, &dcc_chats);
        self.clean_screen(last_conv);
        self.load_messages_on_chat(conversation_label);
    }

    pub fn update_safe_conversation_button(
        &mut self,
        conversation_label: &str,
        dcc_chats: &HashMap<String, DccChat, RandomState>
    ) {
        if self.safe_conversation_button_should_be_disable(conversation_label, dcc_chats) {
            self.disable_safe_conversation_button();
        } else if self.safe_conversation_button_should_be_enable(conversation_label, dcc_chats) {
            self.enable_safe_conversation_button();
        }
    }

    pub fn safe_conversation_button_should_be_disable(
        &mut self,
        conversation_label: &str,
        dcc_chats: &HashMap<String, DccChat, RandomState>
    ) -> bool {
        dcc_chats.contains_key(conversation_label) &&
            self.safe_conversation_button.has_css_class(SAFE_CONVERSATION_BUTTON_CSS)
    }

    pub fn safe_conversation_button_should_be_enable(
        &mut self,
        conversation_label: &str,
        dcc_chats: &HashMap<String, DccChat, RandomState>
    ) -> bool {
        !dcc_chats.contains_key(conversation_label) &&
            self.safe_conversation_button.has_css_class(DISABLE_SAFE_CONVERSATION_BUTTON_CSS)
    }

    pub fn disable_safe_conversation_button(&mut self) {
        self.safe_conversation_button.remove_css_class(SAFE_CONVERSATION_BUTTON_CSS);
        self.safe_conversation_button.add_css_class(DISABLE_SAFE_CONVERSATION_BUTTON_CSS);
        self.safe_conversation_button.set_sensitive(false);
        self.safe_conversation_button.set_has_tooltip(true);
        self.safe_conversation_button.set_tooltip_text(Some(SAFE_BUTTON_TOOLTIP));
    }

    pub fn enable_safe_conversation_button(&mut self) {
        self.safe_conversation_button.remove_css_class(DISABLE_SAFE_CONVERSATION_BUTTON_CSS);
        self.safe_conversation_button.add_css_class(SAFE_CONVERSATION_BUTTON_CSS);
        self.safe_conversation_button.set_sensitive(true);
        self.safe_conversation_button.set_has_tooltip(false);
    }

    fn update_chat_view_when_change_conversation(&mut self, conversation_label: &str) {
        self.quit_channel_button.set_visible(true);
        self.remove_welcome_view_if_any();
        if is_channel(conversation_label) {
            self.set_channel_chat_mode();
        } else {
            self.set_client_chat_mode();
        }
        self.current_chat.set_label(conversation_label);
        self.error_label.set_text("");
        self.input.set_text("");
    }

    fn remove_welcome_view_if_any(&mut self) {
        if self.welcome_box.is_visible() {
            self.remove_welcome_view();
        }
    }

    fn remove_welcome_view(&mut self) {
        self.welcome_box.set_visible(false);
        self.scrollwindow_chat.set_visible(true);
        self.send_message.set_sensitive(true);
        self.input.set_sensitive(true);
    }

    fn update_chat_button_when_clicked(&mut self, conversation_label: &str) {
        let (conversation_button, _) = self.find_button_by_name(conversation_label);
        if let Some(button) = conversation_button {
            remove_button_notifications_if_any(&button, conversation_label);
            select_conversation_button(&button);
        }
    }

    fn update_last_chat_button_when_change_conversation(&mut self, conversation_label: &str) {
        let (conversation_button, _) = self.find_button_by_name(conversation_label);
        if let Some(button) = conversation_button {
            deselect_conversation_button(&button);
        }
    }

    /// Cleans screen messages from conversation.
    fn clean_screen(&mut self, key: String) {
        self.update_screen(key, true);
    }

    /// Loads messages from conversation on the screen.
    fn load_messages_on_chat(&mut self, key: String) {
        self.update_screen(key, false);
    }

    /// Updates screen messages on new conversation.
    fn update_screen(&mut self, key: String, should_remove: bool) {
        let mut prev_message: Vec<Label> = vec![];

        if is_not_empty(&key) && self.messages.contains_key(&key) {
            let messages = self.messages.get(&key).unwrap();
            for message in messages {
                if Self::there_is_sender(message[1].clone(), prev_message) {
                    if should_remove {
                        self.message_box.remove(&message[1]);
                    } else {
                        self.message_box.append(&message[1]);
                    }
                }
                if should_remove {
                    self.message_box.remove(&message[0]);
                } else {
                    self.message_box.append(&message[0]);
                }

                prev_message = message.clone();
            }
        }
    }

    /// Returns bool wether there is a sender or not.
    fn there_is_sender(message: Label, prev_message: Vec<Label>) -> bool {
        message.text() != "" &&
            (prev_message.is_empty() || message.text() != prev_message[1].text())
    }

    /// Creates new notification with message.
    ///
    /// Add it to notifications vec.
    pub fn add_notification(&mut self, message: String) {
        self.notifications.push(message);
        add_notification_to_button(&self.notifications_button, String::from("🔔 notifications"));
    }

    /// Gets all notifications.
    ///
    /// Returns a Vec<String>.
    pub fn get_notifications(&mut self) -> Vec<String> {
        self.notifications.clone()
    }
}