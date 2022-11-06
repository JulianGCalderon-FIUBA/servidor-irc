use gtk4 as gtk;
use gtk::{
    Align,
    Box,
    Orientation,
    prelude::*
};

use super::{MainView};

impl MainView {
    pub fn create_conv_info(&mut self) -> Box {
        let conv_info = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_end(12)
            .halign(Align::Start)
            .build();

        self.quit_channel.add_css_class("exit_channel");
        conv_info.append(&self.quit_channel);

        conv_info.append(&self.channel_info);

        conv_info.append(&self.func_channel);

        conv_info
    }
}