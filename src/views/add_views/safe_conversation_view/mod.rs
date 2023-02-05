/// Contains definition of used requests.
pub mod requests;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk::{
    glib::Sender, prelude::*, Application, ApplicationWindow, Box, Button, Entry, Label,
    ScrolledWindow,
};
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::{
        main_view::widgets_creation::{create_current_chat, create_message_box},
        widgets_creation::{
            build_application_window, create_button_with_margin, create_center_button,
            create_chat_box, create_entry, create_error_label, create_label,
            create_message_sender_box, create_scrollwindow_chat,
        },
        ENTRY_PLACEHOLDER, SEND_BUTTON_TEXT,
    },
};

use self::{
    requests::kick_request,
    widgets_creation::{create_kick_button, create_kick_label},
};

use super::{
    widgets_creation::{create_main_box_add_view, create_title},
    CONTINUE_BUTTON_TEXT,
};

const OPERATOR_FIRST_CHARACTER: &str = "@";
const TITLE: &str = "Members";

/// Shows channel members view.
/// Contains an exit button.
/// Uses sender to communicate with controller.
pub struct SafeConversationView {
    input: Entry,
    message_box: Box,
    scrollwindow_chat: ScrolledWindow,
    error_label: Label,
    send_message: Button,
    current_chat: Label,
    sender: Sender<ControllerMessage>,
}

impl SafeConversationView {
    /// Creates new [`SafeConversationView`]
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            input: create_entry(ENTRY_PLACEHOLDER),
            message_box: create_message_box(),
            scrollwindow_chat: create_scrollwindow_chat(),
            error_label: create_error_label(),
            send_message: create_button_with_margin(SEND_BUTTON_TEXT),
            current_chat: create_current_chat(""),
            sender,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        // let main_box = create_main_box_add_view();

        // main_box.append(&create_title(TITLE));

        // main_box.append(&self.button);

        // self.connect_button(window.clone());

        // window.set_child(Some(&main_box));
        // window

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
        //chat.append(&self.welcome_box);
        chat.append(&self.error_label);
        chat.append(&message_sender_box);

        window.set_child(Some(&chat));
        window
    }

    fn connect_send_button(
        &self,
        input: Entry,
        sender: Sender<ControllerMessage>,
        error_label: Label,
    ) {
        self.send_message.connect_clicked(move |_| {
            // error_label.set_text("");
            // let input_text = input.text();
            // if !entry_is_valid(&input_text, MESSAGE_MAX_CHARACTERS) {
            //     if !input_text.is_empty() {
            //         error_label.set_text(
            //             &format!(
            //                 "{MESSAGE_MAX_CHARACTERS_ERROR} Max: {MESSAGE_MAX_CHARACTERS} characters"
            //             )
            //         );
            //     } else {
            //         error_label.set_text(EMPTY_MESSAGE_ERROR);
            //     }
            //     return;
            // }

            // priv_message_request(input_text, sender.clone());

            // input.set_text("");
        });
    }

    pub fn send_message(&mut self, message: String, nickname: String) {
        // let message = create_send_message(&message);
        // self.message_box.append(&message);
        // adjust_scrollbar(self.scrollwindow_chat.clone());

        // self.messages
        //     .get_mut(&nickname)
        //     .unwrap()
        //     .push(vec![message, create_label("")]);
    }
}
