pub mod requests;
pub mod widgets_creation;

use gtk4::{
    glib::Sender,
    traits::{BoxExt, ButtonExt, EditableExt, WidgetExt},
    Box, Button, Label,
};

use crate::{
    controller::{
        controller_message::ControllerMessage,
        utils::{first_word_of_button, is_channel},
    },
    server::consts::channel::MAX_CHANNELS,
};

use self::{
    requests::{
        add_invite_view_request, add_safe_conversation_view_request, quit_channel_request,
        remove_conversation_request, send_names_request,
    },
    widgets_creation::create_conv_info_box,
};

use super::{MainView, ADD_BUTTON_CSS, DISABLE_BUTTON_CSS};

const EXIT_CHANNEL_BUTTON_CSS: &str = "exit_channel";
pub const SAFE_CONVERSATION_BUTTON_CSS: &str = "safe_conversation";
pub const DISABLE_SAFE_CONVERSATION_BUTTON_CSS: &str = "disable_safe_conversation";
const SEND_MESSAGE_TOOLTIP: &str =
    "Can't send message because there is no chat open! Please open a chat with a client or with a channel";
const INPUT_MESSAGE_TOOLTIP: &str =
    "Can't write a message because there is no chat open! Please open a chat with a client or with a channel";

impl MainView {
    /// Creates conversation info widgets.
    pub fn create_conv_info(&mut self, nickname: &str) -> Box {
        let conv_info = create_conv_info_box();

        self.quit_channel_button
            .add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        self.safe_conversation_button
            .add_css_class(SAFE_CONVERSATION_BUTTON_CSS);

        self.user_info.set_label(nickname);
        conv_info.append(&self.quit_channel_button);
        conv_info.append(&self.channel_members_button);
        conv_info.append(&self.invite_button);
        conv_info.append(&self.safe_conversation_button);

        self.welcome_view();

        self.connect_quit_channel(self.current_chat.clone(), self.sender.clone());
        self.connect_invite_button(self.sender.clone());
        self.connect_safe_conversation_button(self.sender.clone());
        self.connect_members_button(self.sender.clone());

        conv_info
    }

    /// Connects quit channel button.
    ///
    /// Sends quit channel request to the controller.
    fn connect_quit_channel(
        &mut self,
        current_conversation: Label,
        sender: Sender<ControllerMessage>,
    ) {
        self.quit_channel_button.connect_clicked(move |_| {
            if is_channel(&current_conversation.label()) {
                quit_channel_request(sender.clone());
            }
            remove_conversation_request(sender.clone());
        });
    }

    /// Connects invite button.
    ///
    /// Sends invite view request to the controller.
    fn connect_invite_button(&mut self, sender: Sender<ControllerMessage>) {
        self.invite_button.connect_clicked(move |_| {
            add_invite_view_request(sender.clone());
        });
    }

    fn connect_safe_conversation_button(&mut self, sender: Sender<ControllerMessage>) {
        self.safe_conversation_button.connect_clicked(move |_| {
            add_safe_conversation_view_request(sender.clone());
        });
    }

    /// Connects members button.
    ///
    /// Sends names request to the controller.
    fn connect_members_button(&mut self, sender: Sender<ControllerMessage>) {
        self.channel_members_button.connect_clicked(move |_| {
            send_names_request(sender.clone());
        });
    }

    /// Removes conversation from sidebar.
    ///
    /// Removes conversation button.
    pub fn remove_conversation(&mut self, conversation: String) {
        if self.channels_buttons.len() == MAX_CHANNELS {
            self.add_channel.remove_css_class(DISABLE_BUTTON_CSS);
            self.add_channel.add_css_class(ADD_BUTTON_CSS);
        }
        let collection_of_buttons: &mut Vec<Button>;
        let conversation_box: &Box;
        if is_channel(&conversation) {
            collection_of_buttons = &mut self.channels_buttons;
            conversation_box = &mut self.channels_box;
        } else {
            collection_of_buttons = &mut self.clients_buttons;
            conversation_box = &mut self.clients_box;
        }
        let mut counter = 0;
        for button in collection_of_buttons.clone() {
            let button_text = first_word_of_button(&button);
            if button_text == conversation {
                conversation_box.remove(&button);
                break;
            }
            counter += 1;
        }
        if !collection_of_buttons.is_empty() {
            collection_of_buttons.remove(counter);
        }
    }

    /// Shows welcome view.
    ///
    /// Welcome view is used when no conversation is selected.
    pub fn welcome_view(&mut self) {
        self.current_chat.set_label("");
        self.scrollwindow_chat.set_visible(false);
        self.input.set_sensitive(false);
        self.input.set_has_tooltip(true);
        self.input.set_tooltip_text(Some(INPUT_MESSAGE_TOOLTIP));
        self.input.set_text("");
        self.error_label.set_text("");
        self.send_message.set_sensitive(false);
        self.send_message.set_has_tooltip(true);
        self.send_message
            .set_tooltip_text(Some(SEND_MESSAGE_TOOLTIP));
        self.welcome_box.set_visible(true);
        self.quit_channel_button.set_visible(true);
        self.quit_channel_button
            .remove_css_class(EXIT_CHANNEL_BUTTON_CSS);
        self.quit_channel_button.add_css_class(DISABLE_BUTTON_CSS);
        self.invite_button.set_visible(false);
        self.safe_conversation_button.set_visible(false);
        self.channel_members_button.set_visible(false);
        self.safe_conversation_button.set_visible(false);
    }

    /// Returns users channels.
    pub fn get_my_channels(&mut self) -> Vec<String> {
        let mut my_channels: Vec<String> = vec![];
        for channel_button in &self.channels_buttons {
            let button_text = first_word_of_button(channel_button);
            my_channels.push(button_text.to_string());
        }
        my_channels
    }

    /// Returns users clients.
    pub fn get_my_clients(&mut self) -> Vec<String> {
        let mut my_clients: Vec<String> = vec![];
        for client_button in &self.clients_buttons {
            let button_text = first_word_of_button(client_button);
            my_clients.push(button_text.to_string());
        }
        my_clients
    }

    /// Sets view to client chat mode.
    ///
    /// Function is used when a client chat is selected.
    pub fn set_client_chat_mode(&mut self) {
        self.quit_channel_button.set_visible(true);
        if self.quit_channel_button.has_css_class(DISABLE_BUTTON_CSS) {
            self.quit_channel_button
                .remove_css_class(DISABLE_BUTTON_CSS);
            self.quit_channel_button
                .add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        }

        self.invite_button.set_visible(true);
        self.safe_conversation_button.set_visible(true);
        self.channel_members_button.set_visible(false);
    }

    /// Sets view to channel chat mode.
    ///
    /// Function is used when a channel chat is selected.
    pub fn set_channel_chat_mode(&mut self) {
        self.quit_channel_button.set_visible(true);
        if self.quit_channel_button.has_css_class(DISABLE_BUTTON_CSS) {
            self.quit_channel_button
                .remove_css_class(DISABLE_BUTTON_CSS);
            self.quit_channel_button
                .add_css_class(EXIT_CHANNEL_BUTTON_CSS);
        }

        self.invite_button.set_visible(false);
        self.safe_conversation_button.set_visible(false);
        self.channel_members_button.set_visible(true);
    }
}
