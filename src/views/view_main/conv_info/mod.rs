pub mod requests;

use gtk::{glib::Sender, prelude::*, Align, Box, Button, Label, Orientation};
use gtk4 as gtk;

use crate::controller::{controller_handler::is_channel, controller_message::ControllerMessage};

use self::requests::{
    add_invite_view_request, quit_channel_request, remove_conversation_request, send_names_request,
};

use super::{MainView, ADD_BUTTON_CSS, DISABLE_BUTTON_CSS};

const EXIT_CHANNEL_BUTTON_CSS: &str = "exit_channel";

impl MainView {
    pub fn create_conv_info(&mut self, nickname: &str) -> Box {
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

        self.welcome_view();

        self.connect_quit_channel(self.current_chat.clone(), self.sender.clone());
        self.connect_invite_button(self.sender.clone());
        self.connect_members_button(self.sender.clone());

        conv_info
    }

    fn connect_quit_channel(
        &mut self,
        current_conversation: Label,
        sender: Sender<ControllerMessage>,
    ) {
        self.quit_channel_button.connect_clicked(move |_| {
            if is_channel(current_conversation.label().to_string()) {
                quit_channel_request(sender.clone());
            }
            remove_conversation_request(sender.clone());
        });
    }

    fn connect_invite_button(&mut self, sender: Sender<ControllerMessage>) {
        self.invite_button.connect_clicked(move |_| {
            add_invite_view_request(sender.clone());
        });
    }

    fn connect_members_button(&mut self, sender: Sender<ControllerMessage>) {
        self.channel_members_button.connect_clicked(move |_| {
            send_names_request(sender.clone());
        });
    }

    pub fn remove_conversation(&mut self, conversation: String) {
        if self.channels_buttons.len() == 10 {
            self.add_channel.remove_css_class(DISABLE_BUTTON_CSS);
            self.add_channel.add_css_class(ADD_BUTTON_CSS);
        }
        let collection_of_buttons: &mut Vec<Button>;
        let conversation_box: &Box;
        if is_channel(conversation.clone()) {
            collection_of_buttons = &mut self.channels_buttons;
            conversation_box = &mut self.channels_box;
        } else {
            collection_of_buttons = &mut self.clients_buttons;
            conversation_box = &mut self.clients_box;
        }
        let mut counter = 0;
        for button in collection_of_buttons.clone() {
            if button.label().unwrap() == conversation {
                conversation_box.remove(&button);
                break;
            }
            counter += 1;
        }
        if !collection_of_buttons.is_empty() {
            collection_of_buttons.remove(counter);
        }
    }

    pub fn welcome_view(&mut self) {
        self.current_chat.set_label("");
        self.scrollwindow_chat.set_visible(false);
        self.input.set_sensitive(false);
        self.input.set_text("");
        self.error_label.set_text("");
        self.send_message.set_sensitive(false);
        self.welcome_box.set_visible(true);
        self.quit_channel_button.set_visible(true);
        self.quit_channel_button.remove_css_class(EXIT_CHANNEL_BUTTON_CSS);
        self.quit_channel_button.add_css_class(DISABLE_BUTTON_CSS);
        self.invite_button.set_visible(false);
        self.channel_members_button.set_visible(false);
    }

    pub fn get_my_channels(&mut self) -> Vec<String> {
        let mut my_channels: Vec<String> = vec![];
        for channel_button in &self.channels_buttons {
            my_channels.push(channel_button.label().unwrap().to_string());
        }
        my_channels
    }

    pub fn get_my_clients(&mut self) -> Vec<String> {
        let mut my_clients: Vec<String> = vec![];
        for client_button in &self.clients_buttons {
            my_clients.push(client_button.label().unwrap().to_string());
        }
        my_clients
    }

    pub fn set_client_chat_mode(&mut self) {
        self.quit_channel_button.set_visible(true);
        if self.quit_channel_button.has_css_class(DISABLE_BUTTON_CSS) {
            self.quit_channel_button.remove_css_class(DISABLE_BUTTON_CSS);
            self.quit_channel_button.add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        }

        self.invite_button.set_visible(true);
        self.channel_members_button.set_visible(false);
    }

    pub fn set_channel_chat_mode(&mut self) {
        self.quit_channel_button.set_visible(true);
        if self.quit_channel_button.has_css_class(DISABLE_BUTTON_CSS) {
            self.quit_channel_button.remove_css_class(DISABLE_BUTTON_CSS);
            self.quit_channel_button.add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        }

        self.invite_button.set_visible(false);
        self.channel_members_button.set_visible(true);
    }
}
