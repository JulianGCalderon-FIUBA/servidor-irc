pub mod requests;

use gtk::{glib::Sender, prelude::*, Align, Box, Orientation};
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

use self::requests::{quit_channel_request, change_conversation_request, add_invite_view_request};

use super::MainView;

const EXIT_CHANNEL_BUTTON_CSS: &str = "exit_channel";

impl MainView {
    pub fn create_conv_info(&mut self) -> Box {
        let conv_info = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_end(12)
            .halign(Align::Start)
            .build();

        self.quit_channel.add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        self.connect_quit_channel(self.sender.clone());
        conv_info.append(&self.quit_channel);

        conv_info.append(&self.channel_info);

        self.connect_func_channel(self.sender.clone());
        conv_info.append(&self.func_channel);

        conv_info
    }

    fn connect_quit_channel(&mut self, sender: Sender<ControllerMessage>) {
        self.quit_channel.connect_clicked(move |_| {
            quit_channel_request(sender.clone());
            change_conversation_request(sender.clone());
        });
    }

    fn connect_func_channel(&mut self, sender: Sender<ControllerMessage>) {
        self.func_channel.connect_clicked(move |_| {
            add_invite_view_request(sender.clone());
        });
    }

    pub fn remove_channel(&mut self, channel: String) {
        for channel_button in &self.channels_button {
            if channel_button.label().unwrap() == channel {
                self.channels_box.remove(channel_button);
                break;
            }
        }
        if !self.channels_button.is_empty() {
            self.channels_button.remove(0);
        }
    }
}
