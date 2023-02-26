/// Contains definition of used requests.
pub mod requests;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk::{
    glib::Sender,
    traits::{BoxExt, ButtonExt, EditableExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Box, Button, Entry, Label, ScrolledWindow,
};
use gtk4 as gtk;

use crate::{
    controller::{controller_message::ControllerMessage, utils::is_not_empty},
    views::{
        main_view::{
            utils::{adjust_scrollbar, entry_is_valid},
            widgets_creation::{create_current_chat, create_message_box},
        },
        widgets_creation::{
            build_application_window, create_button_with_margin, create_chat_box, create_entry,
            create_error_label, create_message_sender_box, create_scrollwindow_chat,
        },
        ENTRY_PLACEHOLDER, SEND_BUTTON_TEXT,
    },
};

use self::{
    requests::send_safe_message_request,
    widgets_creation::{create_initial_message, create_received_message, create_send_message},
};

const MESSAGE_MAX_CHARACTERS: usize = 60;
const MESSAGE_MAX_CHARACTERS_ERROR: &str = "¡Message too long!";
const EMPTY_MESSAGE_ERROR: &str = "¡Message is empty!";

/// Shows channel members view.
/// Contains an exit button.
/// Uses sender to communicate with controller.
pub struct SafeConversationView {
    input: Entry,
    message_box: Box,
    nickname: String,
    scrollwindow_chat: ScrolledWindow,
    error_label: Label,
    send_message: Button,
    current_chat: Label,
    sender: Sender<ControllerMessage>,
}

impl SafeConversationView {
    /// Creates new [`SafeConversationView`]
    pub fn new(nickname: String, sender: Sender<ControllerMessage>) -> Self {
        Self {
            input: create_entry(ENTRY_PLACEHOLDER),
            message_box: create_message_box(),
            scrollwindow_chat: create_scrollwindow_chat(),
            error_label: create_error_label(),
            nickname,
            send_message: create_button_with_margin(SEND_BUTTON_TEXT),
            current_chat: create_current_chat(""),
            sender,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, client: &str, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let chat = create_chat_box();
        let message_sender_box = create_message_sender_box();

        self.input.set_width_request(600);
        self.input.set_margin_start(15);
        message_sender_box.append(&self.input);

        self.current_chat.set_label(client);

        self.scrollwindow_chat.set_child(Some(&self.message_box));

        self.connect_send_button(
            self.input.clone(),
            self.current_chat.label().to_string(),
            self.sender.clone(),
            self.error_label.clone(),
        );
        message_sender_box.append(&self.send_message);

        let initial_message = create_initial_message(&self.nickname, client);
        self.message_box.append(&initial_message);

        chat.append(&self.current_chat);
        chat.append(&self.scrollwindow_chat);
        chat.append(&self.error_label);
        chat.append(&message_sender_box);

        window.set_child(Some(&chat));
        window
    }

    fn connect_send_button(
        &self,
        input: Entry,
        current_chat: String,
        sender: Sender<ControllerMessage>,
        error_label: Label,
    ) {
        self.send_message.connect_clicked(move |_| {
            error_label.set_text("");
            let input_text = input.text();
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

            send_safe_message_request(
                input_text.to_string(),
                current_chat.to_string(),
                sender.clone(),
            );

            input.set_text("");
        });
    }

    pub fn send_message(&mut self, message: String, receiver: String) {
        if self.current_chat.label() == receiver {
            let message = create_send_message(&message);
            self.message_box.append(&message);
            adjust_scrollbar(self.scrollwindow_chat.clone());
        }
    }

    pub fn receive_message(&mut self, message: String) {
        let message = create_received_message(&message);
        self.message_box.append(&message);
        adjust_scrollbar(self.scrollwindow_chat.clone());
    }
}
