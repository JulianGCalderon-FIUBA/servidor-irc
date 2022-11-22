pub mod requests;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Align, Box, Button, Label, Orientation,
};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

use self::requests::{add_invite_view_request, quit_channel_request, remove_conversation_request};

use super::{requests::change_conversation_request, MainView};

const EXIT_CHANNEL_BUTTON_CSS: &str = "exit_channel";

impl MainView {
    pub fn create_conv_info(&mut self, nickname: &GString) -> Box {
        let conv_info = Box::builder()
            .orientation(Orientation::Vertical)
            .width_request(177)
            .margin_end(12)
            .halign(Align::Start)
            .build();

        self.quit_channel_button
            .add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        self.user_info.set_label(nickname);

        conv_info.append(&self.quit_channel_button);
        conv_info.append(&self.channel_members_button);
        conv_info.append(&self.invite_button);

        self.set_my_chat_mode();

        self.connect_quit_channel(self.current_chat.clone(), self.sender.clone());
        self.connect_invite_button(self.sender.clone());

        conv_info
    }

    fn connect_quit_channel(
        &mut self,
        current_conversation: Label,
        sender: Sender<ControllerMessage>,
    ) {
        let my_nickname = self.user_info.label().unwrap();
        self.quit_channel_button.connect_clicked(move |_| {
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
        let colection_of_buttons: &mut Vec<Button>;
        let conversation_box: &mut Box;
        if Self::current_conv_is_channel(conversation.clone()) {
            colection_of_buttons = &mut self.channels_buttons;
            conversation_box = &mut self.channels_box;
        } else {
            colection_of_buttons = &mut self.clients_buttons;
            conversation_box = &mut self.clients_box;
        }
        let mut counter = 0;
        for button in colection_of_buttons.clone() {
            if button.label().unwrap() == conversation {
                conversation_box.remove(&button);
                break;
            }
            counter += 1;
        }
        if !colection_of_buttons.is_empty() {
            colection_of_buttons.remove(counter);
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
        self.quit_channel_button.set_visible(true);
        self.invite_button.set_visible(true);
        self.channel_members_button.set_visible(false);
    }

    pub fn set_channel_chat_mode(&mut self) {
        self.quit_channel_button.set_visible(true);
        self.invite_button.set_visible(false);
        self.channel_members_button.set_visible(true);
    }

    pub fn set_my_chat_mode(&mut self) {
        self.quit_channel_button.set_visible(true);
        self.invite_button.set_visible(false);
        self.channel_members_button.set_visible(false);
    }
}
