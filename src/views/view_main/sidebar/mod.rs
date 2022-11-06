mod widgets_creation;

use gtk4 as gtk;
use gtk::{
    Box,
    Orientation,
    prelude::*,
};

use self::widgets_creation::create_separator_sidebar;

use super::{MainView};

impl MainView {
    
    pub fn create_sidebar(&mut self) -> Box {
        let sidebar = Box::builder().orientation(Orientation::Vertical).build();

        sidebar.append(&self.channels[0]);
        sidebar.append(&self.channels[1]);

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
