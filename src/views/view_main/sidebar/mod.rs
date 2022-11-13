mod widgets_creation;

use gtk::{ Box, Entry, glib::Sender, Label, Orientation, prelude::*, ScrolledWindow };
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::widgets_creation::{ create_scrollwindow_sidebar, create_separator_sidebar };

use super::{
    MainView,
    utils::{ adjust_scrollbar, entry_is_valid },
    widgets_creation::create_button,
};

impl MainView {
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder().width_request(200).orientation(Orientation::Vertical).build();

        //Channels box
        let scrolled_window_channels: ScrolledWindow = create_scrollwindow_sidebar(&self.channels_box);

        self.connect_add_channel_button(
            self.channels_box.clone(),
            self.input.clone(),
            scrolled_window_channels.clone(),
            self.sender.clone()
        );
        
        //Clients box
        let scrolled_window_clients: ScrolledWindow = create_scrollwindow_sidebar(&self.clients_box);
        
        self.connect_add_client_button(
            self.clients_box.clone(),
            self.input.clone(),
            scrolled_window_clients.clone(),
            self.current_chat.clone(),
            self.sender.clone()
        );

        sidebar.append(&scrolled_window_channels);
        sidebar.append(&self.add_channel);
        sidebar.append(&create_separator_sidebar());
        sidebar.append(&scrolled_window_clients);
        sidebar.append(&self.add_client);

        sidebar
    }

    fn connect_add_channel_button(
        &self,
        channels_box: Box,
        input: Entry,
        scrolled_window: ScrolledWindow,
        sender: Sender<ControllerMessage>
    ) {
        self.add_channel.connect_clicked(move |_| {
            let input_text = input.text();
            if !entry_is_valid(&input_text) {
                return;
            }

            let join_channel_message = ControllerMessage::JoinChannel {
                channel: input_text.clone(),
            };
            sender.send(join_channel_message).expect("Error: join channel command");

            let channel = create_button(&input_text);
            channels_box.append(&channel);

            adjust_scrollbar(scrolled_window.clone());

            input.set_text("");
        });
    }

    fn connect_add_client_button(
        &self,
        clients_box: Box,
        input: Entry,
        scrolled_window: ScrolledWindow,
        current_chat: Label,
        sender: Sender<ControllerMessage>
    ) {
        self.add_client.connect_clicked(move |_| {
            let input_text = input.text();
            if !entry_is_valid(&input_text) {
                return;
            }

            let client = create_button(&input_text);
            current_chat.set_label(&input_text);
            let request = ControllerMessage::ChangeConversation {
                nickname: input_text.to_string(),
            };
            sender.send(request).expect("ERROR: change conversation");
            clients_box.append(&client);

            adjust_scrollbar(scrolled_window.clone());

            input.set_text("");
        });
    }
}