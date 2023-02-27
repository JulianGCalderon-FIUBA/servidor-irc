/// Contains definition of used requests.
pub mod requests;
/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk4::{
    glib::Sender,
    traits::{BoxExt, ButtonExt, EditableExt, WidgetExt},
    Box, Entry, Label,
};

use crate::{
    controller::{controller_message::ControllerMessage, utils::is_not_empty},
    views::{
        main_view::utils::entry_is_valid,
        utils::do_break_line,
        widgets_creation::{
            create_chat_box, create_label, create_message_sender_box, create_received_message,
            create_send_message,
        },
        EMPTY_MESSAGE_ERROR, MESSAGE_MAX_CHARACTERS, MESSAGE_MAX_CHARACTERS_ERROR,
        MESSAGE_MAX_LINE_CHARACTERS,
    },
};

use self::{
    requests::{priv_message_request, send_file_request},
    widgets_creation::create_sender_nickname_label,
};

use super::{
    utils::{add_notification_to_button, adjust_scrollbar},
    MainView,
};

const MESSAGE_SENDER_NAME_CSS: &str = "message_sender_name";

impl MainView {
    /// Creates chat widgets.
    pub fn create_chat(&mut self) -> Box {
        let chat = create_chat_box();
        let message_sender_box = create_message_sender_box();

        message_sender_box.append(&self.send_file_button);
        self.connect_send_file_button(self.sender.clone());

        self.input.set_width_request(600);
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
            let mut input_text = input.text().to_string();
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
            if input_text.len() > MESSAGE_MAX_LINE_CHARACTERS {
                input_text = do_break_line(&input_text);
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

    /// Add a notification to button.
    ///
    /// Receives the name of the button.
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

    /// Connects send file button.
    ///
    /// Sends a send file request.
    fn connect_send_file_button(&self, sender: Sender<ControllerMessage>) {
        self.send_file_button
            .connect_clicked(move |_| send_file_request(sender.clone()));
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

    /// Returns if the received name is the name of the current conversation.
    ///
    /// Receives the name of the conversation, returns a boolean.
    pub fn is_actual_conversation(&mut self, name: &str) -> bool {
        name == self.current_chat.label()
    }

    /// Append a received or send message to the message box.
    ///
    /// Receives the text of the message.
    pub fn append_message(&mut self, message: &Label) {
        self.message_box.append(message);
        adjust_scrollbar(self.scrollwindow_chat.clone());
    }
}
