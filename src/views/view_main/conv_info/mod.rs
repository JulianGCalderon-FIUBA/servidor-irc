pub mod requests;

use gtk::{ glib::{ Sender, GString }, prelude::*, Align, Box, Orientation, Label };
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::requests::{ add_invite_view_request, quit_channel_request, remove_conversation_request };

use super::{ MainView, requests::change_conversation_request };

const EXIT_CHANNEL_BUTTON_CSS: &str = "exit_channel";

impl MainView {
    pub fn create_conv_info(&mut self, nickname: &GString) -> Box {
        let conv_info = Box::builder()
            .orientation(Orientation::Vertical)
            .width_request(177)
            .margin_end(12)
            .halign(Align::Start)
            .build();

        self.quit_channel_button.add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        self.user_info.set_label(&nickname);

        conv_info.append(&self.quit_channel_button);
        conv_info.append(&self.channel_members_button);
        conv_info.append(&self.invite_button);

        self.set_client_chat_mode();
        self.invite_button.set_visible(false);

        self.connect_quit_channel(self.current_chat.clone(), self.sender.clone());
        self.connect_invite_button(self.sender.clone());

        conv_info
    }

    fn connect_quit_channel(&mut self, current_conversation: Label, sender: Sender<ControllerMessage>) {
        let my_nickname = self.user_info.label().unwrap().clone();
        self.quit_channel_button.connect_clicked(move |_| {
            println!("current conv: {}", current_conversation.label());
            
            if Self::current_conv_is_channel(current_conversation.label().to_string()) {
                quit_channel_request(sender.clone());
            }
            remove_conversation_request(sender.clone());
            change_conversation_request(my_nickname.clone(), sender.clone());
        });
    }

    fn connect_invite_button(&mut self, sender: Sender<ControllerMessage>) {
        self.invite_button.connect_clicked(move |_| {
            add_invite_view_request(sender.clone());
        });
    }

    pub fn remove_conversation(&mut self, conversation: String) {
        if Self::current_conv_is_channel(conversation.clone()) {
            for channel_button in &self.channels_buttons {
                if channel_button.label().unwrap() == conversation {
                    self.channels_box.remove(channel_button);
                    if let Some(pos) = self.channels_buttons.iter().position(|x| *x.label().unwrap() == conversation) {
                        self.channels_buttons.remove(pos);
                    }
                    break;
                }
            }
            if !self.channels_buttons.is_empty() {
                self.channels_buttons.remove(0);
            }
        }
        else{
            for client_button in &self.clients_buttons {
                if client_button.label().unwrap() == conversation {
                    self.clients_box.remove(client_button);
                    if let Some(pos) = self.clients_buttons.iter().position(|x| *x.label().unwrap() == conversation) {
                        self.clients_buttons.remove(pos);
                    }
                    break;
                }
            }
            if !self.clients_buttons.is_empty() {
                self.clients_buttons.remove(0);
            }
        }
    }

    pub fn get_my_channels(&mut self) -> Vec<String> {
        let mut my_channels: Vec<String> = vec![];
        for channel_button in &self.channels_buttons {
            my_channels.push(channel_button.label().unwrap().to_string());
        }
        my_channels
    }

    pub fn set_client_chat_mode(&mut self) {
        self.invite_button.set_visible(true);
        self.channel_members_button.set_visible(false);
    }

    pub fn set_channel_chat_mode(&mut self) {
        self.invite_button.set_visible(false);
        self.channel_members_button.set_visible(true);
    }
}