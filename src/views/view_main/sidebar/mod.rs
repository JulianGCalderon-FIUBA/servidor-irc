mod widgets_creation;

use gtk::{ glib::Sender, prelude::*, Box, Button, Label, Orientation, Entry, ScrolledWindow };
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::widgets_creation::create_separator_sidebar;

use super::{MainView, widgets_creation::create_button};

impl MainView {
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder().width_request(200).orientation(Orientation::Vertical).build();

        let scrolled_window: ScrolledWindow = ScrolledWindow::builder()
        .min_content_height(320)
        .child(&self.channels_box)
        .build();
        sidebar.append(&scrolled_window);

        self.add_channel.add_css_class("add");
        sidebar.append(&self.add_channel);
        self.connect_add_button(
            self.channels_box.clone(),
            self.input.clone(),
            scrolled_window.clone(),
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
            self.clients_box.append(button);
        }
        
        let scrolled_window: ScrolledWindow = ScrolledWindow::builder()
        .min_content_height(320)
        .child(&self.clients_box)
        .build();
        sidebar.append(&scrolled_window);

        self.add_client.add_css_class("add");
        sidebar.append(&self.add_client);

        sidebar
    }

    fn connect_add_button(
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
            sender
                .send(join_channel_message)
                .expect("Error: join channel command");


            let channel = create_button(&input_text);
            channels_box.append(&channel);

            let adj = scrolled_window.vadjustment();
            adj.set_upper(adj.upper() + adj.page_size());
            adj.set_value(adj.upper());
            scrolled_window.set_vadjustment(Some(&adj));

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
