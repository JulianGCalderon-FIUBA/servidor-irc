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
    controller::controller_message::ControllerMessage,
    views::{
        main_view::{
            utils::adjust_scrollbar,
            widgets_creation::{create_current_chat, create_message_box},
        },
        utils::do_break_line,
        widgets_creation::{
            build_application_window, create_button_with_margin, create_chat_box, create_entry,
            create_error_label, create_message_sender_box, create_scrollwindow_chat,
        },
        EMPTY_MESSAGE_ERROR, ENTRY_PLACEHOLDER, MESSAGE_MAX_LINE_CHARACTERS, SEND_BUTTON_TEXT,
    },
};

use self::{
    requests::send_safe_message_request,
    widgets_creation::{create_initial_message, create_received_message, create_send_message},
};

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
            let mut input_text = input.text().to_string();
            if input_text.is_empty() {
                error_label.set_text(EMPTY_MESSAGE_ERROR);
                return;
            }
            if input_text.len() > MESSAGE_MAX_LINE_CHARACTERS {
                input_text = do_break_line(&input_text);
            }

            send_safe_message_request(input_text, current_chat.to_string(), sender.clone());

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
