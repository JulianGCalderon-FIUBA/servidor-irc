pub mod requests;
mod widgets_creation;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Box, Button, Label, Orientation,
};
use gtk4 as gtk;

use crate::{
    controller::{controller_handler::is_channel, controller_message::ControllerMessage},
    views::{
        views_add::widget_creations::create_title, widgets_creation::create_button_with_margin,
    },
};

use self::{
    requests::{add_view_to_add_client_request, send_list_request},
    widgets_creation::create_separator_sidebar,
};

use super::{requests::change_conversation_request, utils::adjust_scrollbar, MainView};

const CHANNELS_TITLE: &str = "Channels";
const CLIENTS_TITLE: &str = "Clients";

impl MainView {
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

        sidebar.append(&channels_title);
        sidebar.append(&self.scrollwindow_channels);
        sidebar.append(&self.add_channel);
        sidebar.append(&create_separator_sidebar());
        sidebar.append(&clients_title);
        sidebar.append(&self.scrollwindow_clients);
        sidebar.append(&self.add_client);

        sidebar
    }

    fn connect_add_client_button(&self, button: Button, sender: Sender<ControllerMessage>) {
        button.connect_clicked(move |_| {
            add_view_to_add_client_request(sender.clone());
        });
    }

    pub fn connect_add_channel_button(&self, button: Button, sender: Sender<ControllerMessage>) {
        button.connect_clicked(move |_| {
            send_list_request(sender.clone());
        });
    }

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

        println!("Added to {}", channel);

        self.messages.insert(channel.to_string(), vec![]);
        adjust_scrollbar(self.scrollwindow_channels.clone());
    }

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

        self.messages.insert(client.to_string(), vec![]);

        adjust_scrollbar(self.scrollwindow_clients.clone());
    }

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

    pub fn change_conversation(&mut self, last_conv: String, conversation_label: String) {
        self.current_chat.set_label(&conversation_label);
        self.scrollwindow_chat.set_visible(true);
        self.send_message.set_sensitive(true);
        self.welcome_box.set_visible(false);

        self.clean_screen(last_conv);
        self.load_messages_on_chat(conversation_label.clone());

        self.quit_channel_button.set_visible(true);
        if is_channel(conversation_label.clone()) {
            self.set_channel_chat_mode();
        } else if conversation_label == self.user_info.label().unwrap() {
            self.set_my_chat_mode();
        } else {
            self.set_client_chat_mode();
        }
    }

    fn clean_screen(&mut self, key: String) {
        self.update_screen(key, true);
    }

    fn load_messages_on_chat(&mut self, key: String) {
        self.update_screen(key, false);
    }

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

    fn there_is_sender(message: Label, prev_message: Vec<Label>) -> bool {
        message.text() != ""
            && (prev_message.is_empty() || message.text() != prev_message[1].text())
    }
}
