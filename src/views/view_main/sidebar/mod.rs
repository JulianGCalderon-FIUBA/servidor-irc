mod widgets_creation;

use gtk::{ glib::Sender, prelude::*, Box, Button, Label, Orientation, Entry };
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::widgets_creation::create_separator_sidebar;

use super::{MainView, widgets_creation::create_button};

impl MainView {
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder().orientation(Orientation::Vertical).build();

        for button in &self.channels {
            self.channels_box.append(button);
        }
        sidebar.append(&self.channels_box);

        self.add_channel.add_css_class("add");
        sidebar.append(&self.add_channel);
        self.connect_add_button(
            self.channels_box.clone(),
            self.input.clone(),
            self.sender.clone()
        );

        let separator = create_separator_sidebar();
        sidebar.append(&separator);

        let current_chat_clone = self.current_chat.clone();
        for button in &self.clients {
            let label = button.label().unwrap().to_string();
            self.connect_conv_button(
                button,
                label.clone(),
                current_chat_clone.clone(),
                self.sender.clone()
            );
            sidebar.append(button);
        }
        self.add_client.add_css_class("add");
        sidebar.append(&self.add_client);

        sidebar
    }

    fn connect_add_button(
        &self,
        channels_box: Box,
        input: Entry,
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
            sender
                .send(join_channel_message)
                .expect("Error: join channel command");


            let channel = create_button(&input_text);
            channels_box.append(&channel);

            input.set_text("");
        });
    }

    fn connect_conv_button(
        &self,
        button: &Button,
        label: String,
        current_chat: Label,
        sender: Sender<ControllerMessage>
    ) {
        button.connect_clicked(move |_| {
            current_chat.set_label(&label);
            let request = ControllerMessage::ChangeConversation {
                nickname: label.clone(),
            };
            sender.send(request).expect("ERROR: change conversation");
        });
    }
}

fn entry_is_valid(entry_text: &str) -> bool {
    !entry_text.is_empty()
}
