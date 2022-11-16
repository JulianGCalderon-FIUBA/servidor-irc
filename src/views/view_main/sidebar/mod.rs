mod widgets_creation;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Box, Button, Orientation,
};
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::views_add::widget_creations::create_title,
};

use self::widgets_creation::create_separator_sidebar;

use super::{utils::adjust_scrollbar, widgets_creation::create_button, MainView};

impl MainView {
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder()
            .width_request(200)
            .orientation(Orientation::Vertical)
            .build();

        //Channels box
        let channels_title = create_title("channels");

        self.scrollwindow_channels
            .set_child(Some(&self.channels_box));

        self.connect_add_button(self.add_channel.clone(), false, self.sender.clone());

        //Clients box
        let clients_title = create_title("clients");

        self.scrollwindow_clients.set_child(Some(&self.clients_box));

        self.connect_add_button(self.add_client.clone(), true, self.sender.clone());

        sidebar.append(&channels_title);
        sidebar.append(&self.scrollwindow_channels);
        sidebar.append(&self.add_channel);
        sidebar.append(&create_separator_sidebar());
        sidebar.append(&clients_title);
        sidebar.append(&self.scrollwindow_clients);
        sidebar.append(&self.add_client);

        sidebar
    }

    fn connect_add_button(
        &self,
        button: Button,
        is_add_client: bool,
        sender: Sender<ControllerMessage>,
    ) {
        button.connect_clicked(move |_| {
            let add_view = if is_add_client {
                ControllerMessage::AddViewToAddClient {}
            } else {
                ControllerMessage::AddViewToAddChannel {}
            };
            sender.send(add_view).expect("ERROR");
        });
    }

    pub fn add_channel(&mut self, channel: GString) {
        let join_channel_message = ControllerMessage::JoinChannel {
            channel: channel.clone(),
        };
        self.sender
            .send(join_channel_message)
            .expect("Error: join channel command");

        Self::change_conversation_request(channel.clone(), self.sender.clone());
        let channel_button = create_button(&channel);
        self.connect_channel_client_button(channel_button.clone(), channel, self.sender.clone());
        self.channels_box.append(&channel_button);
        self.channels_button.push(channel_button);

        adjust_scrollbar(self.scrollwindow_channels.clone());
    }


    pub fn add_client(&mut self, client: GString) {
        Self::change_conversation_request(client.clone(), self.sender.clone());
        let client_button = create_button(&client);
        self.connect_channel_client_button(client_button.clone(), client, self.sender.clone());
        self.clients_box.append(&client_button);

        adjust_scrollbar(self.scrollwindow_clients.clone());
    }

    pub fn connect_channel_client_button(
        &self,
        button: Button,
        channel_or_client: GString,
        sender: Sender<ControllerMessage>,
    ) {
        button.connect_clicked(move |_| {
            Self::change_conversation_request(channel_or_client.clone(), sender.clone());
        });
    }

    pub fn change_conversation_request(conversation: GString, sender: Sender<ControllerMessage>) {
        let request = ControllerMessage::ChangeConversation {
            nickname: conversation.to_string(),
        };
        sender.send(request).expect("ERROR: change conversation");
    }


    pub fn change_conversation(&mut self, conversation_label: String) {
        self.current_chat.set_label(&conversation_label);
        for message in &self.messages {
            self.message_box.remove(message);
        }
    }
}
