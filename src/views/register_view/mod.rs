/// Contains definition of used requests.
pub mod requests;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Application, ApplicationWindow, Button, Entry, Label, Orientation, PasswordEntry,
};
use gtk4 as gtk;

use self::requests::register_request;

use super::{
    widgets_creation::{
        build_application_window, create_center_button, create_entry, create_error_label,
        create_label_input_box, create_main_box, create_password_entry,
    },
    MAIN_BOX_CSS, NICKNAME_LABEL_TEXT, PASSWORD_LABEL_TEXT, REALNAME_LABEL_TEXT,
    USERNAME_LABEL_TEXT,
};

use crate::controller::{controller_message::ControllerMessage, utils::is_not_empty};

const LOGIN_BUTTON_TEXT: &str = "Login";
const ERR_FIELDS_REQUIRED: &str = "¡All fields are required!";
const FIELD_MAX_CHARACTERS: usize = 9;
const FIELD_MAX_CHARACTERS_ERROR: &str = "¡Fields are too long!";

/// Shows registation view.
/// Contains a realname, nickname, username and password entry.
/// Uses sender to communicate with controller.
pub struct RegisterView {
    error_label: Label,
    login_button: Button,
    nick_entry: Entry,
    pass_entry: PasswordEntry,
    realname_entry: Entry,
    sender: Sender<ControllerMessage>,
    username_entry: Entry,
}

impl RegisterView {
    /// Creates new [`RegisterView`]
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            error_label: create_error_label(),
            login_button: create_center_button(LOGIN_BUTTON_TEXT),
            nick_entry: create_entry(""),
            pass_entry: create_password_entry(""),
            realname_entry: create_entry(""),
            sender,
            username_entry: create_entry(""),
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box(Orientation::Vertical, 300, 300);
        main_box.add_css_class(MAIN_BOX_CSS);

        let realname_box = create_label_input_box(REALNAME_LABEL_TEXT);
        realname_box.append(&self.realname_entry);
        main_box.append(&realname_box);

        let nickname_box = create_label_input_box(NICKNAME_LABEL_TEXT);
        nickname_box.append(&self.nick_entry);
        main_box.append(&nickname_box);

        let username_box = create_label_input_box(USERNAME_LABEL_TEXT);
        username_box.append(&self.username_entry);
        main_box.append(&username_box);

        let password_box = create_label_input_box(PASSWORD_LABEL_TEXT);
        password_box.append(&self.pass_entry);
        main_box.append(&password_box);

        main_box.append(&self.login_button);

        self.error_label.set_margin_bottom(10);
        main_box.append(&self.error_label);

        self.connect_button(
            self.realname_entry.clone(),
            self.pass_entry.clone(),
            self.nick_entry.clone(),
            self.username_entry.clone(),
            self.error_label.clone(),
            self.sender.clone(),
        );

        window.set_child(Some(&main_box));

        window
    }

    /// Connects connect button.
    ///
    /// Sends register request to the controller.
    fn connect_button(
        &self,
        realname_entry: Entry,
        pass_entry: PasswordEntry,
        nick_entry: Entry,
        username_entry: Entry,
        error_label: Label,
        sender: Sender<ControllerMessage>,
    ) {
        self.login_button.connect_clicked(move |_| {
            error_label.set_text("");
            let pass = pass_entry.text();
            let nickname = nick_entry.text();
            let username = username_entry.text();
            let realname = realname_entry.text();

            if Self::register_fiels_are_valid(&pass, &nickname, &username, &realname) {
                register_request(pass, nickname, username, realname, sender.clone());
            } else if nickname.len() > FIELD_MAX_CHARACTERS
                || realname.len() > FIELD_MAX_CHARACTERS
                || username.len() > FIELD_MAX_CHARACTERS
                || pass.len() > FIELD_MAX_CHARACTERS
            {
                error_label.set_text(&format!(
                    "{FIELD_MAX_CHARACTERS_ERROR} Max: {FIELD_MAX_CHARACTERS} characters"
                ));
            } else {
                error_label.set_text(ERR_FIELDS_REQUIRED);
            }
        });
    }

    /// Checks if entrys are not empty.
    ///
    /// Returns a bool.
    fn register_fiels_are_valid(
        pass: &GString,
        nickname: &GString,
        username: &GString,
        realname: &GString,
    ) -> bool {
        is_not_empty(realname)
            && is_not_empty(pass)
            && is_not_empty(nickname)
            && is_not_empty(username)
            && pass.len() < FIELD_MAX_CHARACTERS
            && nickname.len() < FIELD_MAX_CHARACTERS
            && nickname.len() < FIELD_MAX_CHARACTERS
            && realname.len() < FIELD_MAX_CHARACTERS
    }
}
