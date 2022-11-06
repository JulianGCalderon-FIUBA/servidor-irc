mod widgets_creation;

use gtk::{ prelude::*, Box, Orientation };
use gtk4 as gtk;

use self::widgets_creation::create_separator_sidebar;

use super::MainView;

impl MainView {
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder().orientation(Orientation::Vertical).build();

        sidebar.append(&self.channels[0]);
        sidebar.append(&self.channels[1]);

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

        sidebar.append(&self.clients[0]);
        sidebar.append(&self.clients[1]);
        sidebar.append(&self.clients[2]);
        sidebar.append(&self.clients[3]);

        self.add_client.add_css_class("add");
        sidebar.append(&self.add_client);
        sidebar
    }
}