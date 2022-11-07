mod widgets_creation;

use gtk::{ prelude::*, Box, Orientation, Button, Label};
use gtk4 as gtk;

use self::widgets_creation::create_separator_sidebar;

use super::MainView;

impl MainView {
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder().orientation(Orientation::Vertical).build();

        // self.connect_conv_buttons();

        for button in &self.channels {
            sidebar.append(button);
        }
        

        // sidebar.append(&self.channels[0]);
        // sidebar.append(&self.channels[1]);

        // let mut current_conversation = &mut self.current_conversation;
        // let mut channel_text = self.channels[0].label().unwrap().to_string().clone();
        // // self.channels[0].connect_clicked( move |_| {
        //     self.change_current_conversation(channel_text);
        // });
        //for channel in self.channels {
        //     let channel_text = channel.label().unwrap().to_string().clone();
        //     channel.connect_clicked(move |_| {
        //         self.current_conversation = "hola".to_string();
        //     });
        // }
        self.add_channel.add_css_class("add");
        sidebar.append(&self.add_channel);

        let separator = create_separator_sidebar();
        sidebar.append(&separator);

        let current_chat_clone = self.current_chat.clone();

        for button in &self.clients {
            let label = button.label().unwrap().to_string();
            self.connect_conv_button(button, label, current_chat_clone.clone());
            sidebar.append(button);
        }

        self.current_chat.set_label(&current_chat_clone.label().to_string());

        let label = self.current_chat.label().to_string();
        self.add_client.connect_clicked(move |_| {
            println!("label says: {}", label);
        });

        // sidebar.append(&self.clients[0]);
        // sidebar.append(&self.clients[1]);
        // sidebar.append(&self.clients[2]);
        // sidebar.append(&self.clients[3]);

        self.add_client.add_css_class("add");
        sidebar.append(&self.add_client);
        sidebar
    }

    fn connect_conv_button(&self, button: &Button, label: String, current_chat: Label) {
        button.connect_clicked(move |_| {
            current_chat.set_label(&label);
        });
    }
}