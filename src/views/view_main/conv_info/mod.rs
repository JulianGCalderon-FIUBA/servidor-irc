pub mod requests;

use gtk::{ glib::{Sender, GString}, prelude::*, Align, Box, Orientation };
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::requests::{ add_invite_view_request, change_conversation_request, quit_channel_request };

use super::MainView;

const EXIT_CHANNEL_BUTTON_CSS: &str = "exit_channel";

impl MainView {
    pub fn create_conv_info(&mut self, nickname: &GString) -> Box {
        let conv_info = Box::builder()
            .orientation(Orientation::Vertical)
            .width_request(100)
            .margin_end(12)
            .halign(Align::Start)
            .build();
        
        self.quit_channel.add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        self.user_info.set_label(&nickname);

        self.invite_button.set_focusable(true);
        conv_info.append(&self.quit_channel);
        conv_info.append(&self.channel_info);
        conv_info.append(&self.invite_button);

        
        self.set_client_chat_mode();
        self.invite_button.set_visible(false);

        self.connect_quit_channel(self.sender.clone());
        self.connect_invite_button(self.sender.clone());

        conv_info
    }


    fn connect_quit_channel(&mut self, sender: Sender<ControllerMessage>) {
        self.quit_channel.connect_clicked(move |_| {
            quit_channel_request(sender.clone());
            change_conversation_request(sender.clone());
        });
    }

    fn connect_invite_button(&mut self, sender: Sender<ControllerMessage>) {
        self.invite_button.connect_clicked(move |_| {
            add_invite_view_request(sender.clone());
        });
    }

    pub fn remove_channel(&mut self, channel: String) {
        for channel_button in &self.channels_buttons {
            if channel_button.label().unwrap() == channel {
                self.channels_box.remove(channel_button);
                break;
            }
        }
        if !self.channels_buttons.is_empty() {
            self.channels_buttons.remove(0);
        }
    }

    pub fn get_my_channels(&mut self) -> Vec<String> {
        let mut my_channels: Vec<String> = vec![];
        for channel_button in &self.channels_buttons {
            my_channels.push(channel_button.label().unwrap().to_string());
        }
        my_channels
    }

    pub fn set_client_chat_mode(&mut self){
        self.invite_button.set_visible(true);
    }

    pub fn set_channel_chat_mode(&mut self){
        self.invite_button.set_visible(false);
    }
}