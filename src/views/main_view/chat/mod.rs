pub mod requests;
pub mod widgets_creation;

use gtk::{
    glib::Sender,
    traits::{BoxExt, ButtonExt, EditableExt, WidgetExt},
    Box, Entry, Label,
};
use gtk4 as gtk;

use crate::{
    controller::{controller_message::ControllerMessage, utils::is_not_empty},
    views::{
        main_view::utils::entry_is_valid,
        widgets_creation::{create_chat_box, create_label, create_message_sender_box},
    },
};

use self::{
    requests::priv_message_request,
    widgets_creation::{
        create_received_message, create_send_message, create_sender_nickname_label,
    },
};

use super::{
    utils::{add_notification_to_button, adjust_scrollbar},
    MainView,
};

const MESSAGE_MAX_CHARACTERS: usize = 60;
const MESSAGE_MAX_CHARACTERS_ERROR: &str = "¡Message too long!";
const EMPTY_MESSAGE_ERROR: &str = "¡Message is empty!";

const RECEIVED_MESSAGE_CSS: &str = "received_message";
const SEND_MESSAGE_CSS: &str = "send_message";
const MESSAGE_SENDER_NAME_CSS: &str = "message_sender_name";

impl MainView {
    /// Creates chat widgets.
    pub fn create_chat(&mut self) -> Box {
        let chat = create_chat_box();
        let message_sender_box = create_message_sender_box();

        self.input.set_width_request(600);
        self.input.set_margin_start(15);
        message_sender_box.append(&self.input);

        self.scrollwindow_chat.set_child(Some(&self.message_box));
        self.scrollwindow_chat.set_visible(false);

        self.connect_send_button(
            self.input.clone(),
            self.sender.clone(),
            self.error_label.clone(),
        );
        message_sender_box.append(&self.send_message);

        chat.append(&self.current_chat);
        chat.append(&self.scrollwindow_chat);
        chat.append(&self.welcome_box);
        chat.append(&self.error_label);
        chat.append(&message_sender_box);
        chat
    }

    /// Connects send button.
    ///
    /// Sends a private message request to the controller.
    fn connect_send_button(
        &self,
        input: Entry,
        sender: Sender<ControllerMessage>,
        error_label: Label,
    ) {
        self.send_message.connect_clicked(move |_| {
            error_label.set_text("");
            let input_text = input.text().to_string();
            if !entry_is_valid(&input_text, MESSAGE_MAX_CHARACTERS) {
                if is_not_empty(&input_text) {
                    error_label.set_text(&format!(
                        "{MESSAGE_MAX_CHARACTERS_ERROR} Max: {MESSAGE_MAX_CHARACTERS} characters"
                    ));
                } else {
                    error_label.set_text(EMPTY_MESSAGE_ERROR);
                }
                return;
            }

            priv_message_request(input_text, sender.clone());

            input.set_text("");
        });
    }

    /// Creates a new message in a channel chat.
    ///
    /// Function is used when a channel message is received.
    pub fn receive_priv_channel_message(
        &mut self,
        message_text: String,
        sender_nickname: String,
        channel: String,
    ) {
        if sender_nickname == self.user_info.label().unwrap() {
            return;
        }

        let sender_nickname_label = create_sender_nickname_label(&sender_nickname);
        let message = create_received_message(&message_text);

        if self.is_actual_conversation(&channel) {
            if self.should_show_nickname(&channel, sender_nickname) {
                self.message_box.append(&sender_nickname_label);
            }
            self.append_message(&message);
        } else {
            self.add_notification_to_button(channel.clone());
        }

        self.messages
            .get_mut(&channel)
            .unwrap()
            .push(vec![message, sender_nickname_label]);
    }

    pub fn add_notification_to_button(&mut self, conv_name: String) {
        let (finded_button, name) = self.find_button_by_name(&conv_name);
        if let Some(button) = finded_button {
            add_notification_to_button(&button, name);
        }
    }

    /// Creates a new message in a client chat.
    ///
    /// Function is used when a client message is received.
    pub fn receive_priv_client_message(&mut self, message_text: String, nickname: String) {
        let message_label = create_received_message(&message_text);

        if self.messages.get_mut(&nickname).is_none() {
            self.add_client(&nickname);
        }
        let messages = self.messages.get_mut(&nickname).unwrap();
        messages.push(vec![
            message_label.clone(),
            create_sender_nickname_label(""),
        ]);

        if self.is_actual_conversation(&nickname) {
            self.append_message(&message_label);
        } else {
            self.add_notification_to_button(nickname.clone());
        }
    }

    /// Creates sent message label in the chat.
    pub fn send_message(&mut self, message: String) {
        let nickname: String = self.current_chat.label().to_string();
        let message = create_send_message(&message);
        self.append_message(&message);

        self.messages
            .get_mut(&nickname)
            .unwrap()
            .push(vec![message, create_label("")]);
    }

    /// Returns bool if the messages should be shown.
    ///
    /// If it is received by the sender, returns false.
    pub fn should_show_nickname(&mut self, channel: &str, sender_nickname: String) -> bool {
        let messages = self.messages.get(channel);
        Self::prev_message_has_different_sender(messages, sender_nickname)
            || messages.unwrap().is_empty()
    }

    /// Returns bool, whether the previous message was sent by a different sender.
    pub fn prev_message_has_different_sender(
        messages: Option<&Vec<Vec<gtk4::Label>>>,
        sender_nickname: String,
    ) -> bool {
        messages.is_some()
            && messages.unwrap().last().is_some()
            && messages.unwrap().last().unwrap()[1].text() != sender_nickname
    }

    pub fn is_actual_conversation(&mut self, name: &str) -> bool {
        name == self.current_chat.label()
    }

    pub fn append_message(&mut self, message: &Label) {
        self.message_box.append(message);
        adjust_scrollbar(self.scrollwindow_chat.clone());
    }
}
