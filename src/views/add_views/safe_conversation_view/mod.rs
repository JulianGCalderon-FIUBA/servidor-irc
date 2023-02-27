/// Contains definition of used requests.
pub mod requests;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk4::{
    glib::Sender,
    traits::{BoxExt, ButtonExt, EditableExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Box, Button, Entry, Label, ScrolledWindow,
};

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
            create_error_label, create_message_sender_box, create_received_message,
            create_scrollwindow_chat, create_send_message,
        },
        EMPTY_MESSAGE_ERROR, ENTRY_PLACEHOLDER, MESSAGE_MAX_LINE_CHARACTERS, SEND_BUTTON_TEXT,
    },
};

use self::{requests::{send_safe_message_request, close_safe_view_request}, widgets_creation::create_initial_message};

const QUIT_BUTTON_TEXT: &str = "x";
const QUIT_BUTTON_CSS: &str = "exit_channel";

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
    close_button: Button,
    current_chat: Label,
    sender: Sender<ControllerMessage>,
}

impl SafeConversationView {
    /// Creates new [`SafeConversationView`]
    pub fn new(client: &str, nickname: String, sender: Sender<ControllerMessage>) -> Self {
        Self {
            input: create_entry(ENTRY_PLACEHOLDER),
            message_box: create_message_box(),
            scrollwindow_chat: create_scrollwindow_chat(),
            error_label: create_error_label(),
            nickname,
            send_message: create_button_with_margin(SEND_BUTTON_TEXT),
            close_button: create_button_with_margin(QUIT_BUTTON_TEXT),
            current_chat: create_current_chat(client),
            sender,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let top_box = create_message_sender_box();

        self.close_button
            .add_css_class(QUIT_BUTTON_CSS);

        top_box.append(&self.current_chat);
        top_box.append(&self.close_button);

        let chat = create_chat_box();
        let message_sender_box = create_message_sender_box();

        self.input.set_width_request(600);
        self.input.set_margin_start(15);
        message_sender_box.append(&self.input);

        self.scrollwindow_chat.set_child(Some(&self.message_box));

        self.connect_send_button(
            self.input.clone(),
            self.current_chat.label().to_string(),
            self.sender.clone(),
            self.error_label.clone(),
        );
        message_sender_box.append(&self.send_message);

        let initial_message = create_initial_message(&self.nickname, &self.current_chat.label().to_string());
        self.message_box.append(&initial_message);

        chat.append(&top_box);
        chat.append(&self.scrollwindow_chat);
        chat.append(&self.error_label);
        chat.append(&message_sender_box);

        window.set_child(Some(&chat));

        self.connect_close_button(self.current_chat.label().to_string(), self.sender.clone());

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

    /// Closes the view.
    fn connect_close_button(&mut self, current_chat: String, sender: Sender<ControllerMessage>) {
        self.close_button.connect_clicked(move |_| {
            close_safe_view_request(current_chat.clone(), sender.clone());
        });
    }
}
