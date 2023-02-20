pub mod requests;
mod widgets_creation;

use gtk::{glib::Sender, prelude::*, Box, Button, Label, Orientation};
use gtk4 as gtk;

use crate::{
    controller::{controller_message::ControllerMessage, interface_controller::utils::is_channel},
    views::{
        add_views::widgets_creation::create_title,
        main_view::{ADD_BUTTON_CSS, DISABLE_BUTTON_CSS},
        widgets_creation::create_button_with_margin,
    },
};

use self::{
    requests::{
        add_notifications_view_request, add_user_info_view, add_view_to_add_client_request,
        send_list_request,
    },
    widgets_creation::create_separator_sidebar,
};

use super::{
    requests::change_conversation_request,
    utils::{adjust_scrollbar, add_notification_to_button},
    MainView, NO_NOTIFICATIONS_TEXT,
};

const CHANNELS_TITLE: &str = "Channels";
const CLIENTS_TITLE: &str = "Clients";
const NOTIFICATIONS_CSS: &str = "notifications_button_on";

impl MainView {
    /// Creates sidebar widgets.
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder()
            .width_request(200)
            .orientation(Orientation::Vertical)
            .build();

        //Channels box
        let channels_title = create_title(CHANNELS_TITLE);
        self.scrollwindow_channels
            .set_child(Some(&self.channels_box));
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
            Self::remove_unread_notifications(button_clone.clone());
        });
    }

    /// Adds channel to the sidebar.  
    ///
    /// Creates new channel button.
    pub fn add_channel(&mut self, channel: String) {
        change_conversation_request(channel.clone(), self.sender.clone());
        let channel_button = create_button_with_margin(&channel);
        self.connect_channel_client_button(
            channel_button.clone(),
            channel.clone(),
            self.sender.clone(),
        );
        self.channels_box.append(&channel_button);
        self.channels_buttons.push(channel_button);
        if self.channels_buttons.len() >= 10 {
            self.add_channel.remove_css_class(ADD_BUTTON_CSS);
            self.add_channel.add_css_class(DISABLE_BUTTON_CSS);
        }

        self.messages.insert(channel, vec![]);
        adjust_scrollbar(self.scrollwindow_channels.clone());
    }

    /// Adds client to the sidebar.  
    ///
    /// Creates new client button.
    pub fn add_client(&mut self, client: String) {
        change_conversation_request(client.clone(), self.sender.clone());
        let client_button = create_button_with_margin(&client);
        self.connect_channel_client_button(
            client_button.clone(),
            client.clone(),
            self.sender.clone(),
        );
        self.clients_box.append(&client_button);
        self.clients_buttons.push(client_button);

        self.messages.insert(client, vec![]);

        adjust_scrollbar(self.scrollwindow_clients.clone());
    }

    /// Connects conversation button.
    ///
    /// Sends change conversation request to the controller.
    pub fn connect_channel_client_button(
        &self,
        button: Button,
        channel_or_client: String,
        sender: Sender<ControllerMessage>,
    ) {
        button.connect_clicked(move |_| {
            change_conversation_request(channel_or_client.clone(), sender.clone());
        });
    }

    /// Changes conversation view.  
    ///
    /// Changes chat label and messages.  
    pub fn change_conversation(&mut self, conversation_label: String) {
        self.current_chat.set_label(&conversation_label);
        self.scrollwindow_chat.set_visible(true);
        self.send_message.set_sensitive(true);
        self.input.set_sensitive(true);
        self.error_label.set_text("");
        self.input.set_text("");

        self.welcome_box.set_visible(false);

        self.clean_screen(self.current_chat.label().to_string());
        self.load_messages_on_chat(conversation_label.clone());

        self.quit_channel_button.set_visible(true);
        if is_channel(conversation_label) {
            self.set_channel_chat_mode();
        } else {
            self.set_client_chat_mode();
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

        if self.messages.contains_key(&key) {
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
        message.text() != ""
            && (prev_message.is_empty() || message.text() != prev_message[1].text())
    }

    /// Creates new notification with message.  
    ///
    /// Add it to notifications vec.
    pub fn add_notification(&mut self, message: String) {
        self.notifications.push(message);
        add_notification_to_button(&self.notifications_button, String::from("🔔 notifications"));
    }

    /// Sets number of notifications to 0.
    ///
    /// Functions is used when notifications are read.
    pub fn remove_unread_notifications(button: Button) {
        button.set_label(NO_NOTIFICATIONS_TEXT);
        if button.has_css_class(NOTIFICATIONS_CSS) {
            button.remove_css_class(NOTIFICATIONS_CSS)
        }
    }

    /// Gets all notifications.  
    ///
    /// Returns a Vec<String>.
    pub fn get_notifications(&mut self) -> Vec<String> {
        self.notifications.clone()
    }
}
