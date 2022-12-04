pub mod requests;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Application, ApplicationWindow, Button, Entry, Orientation,
};
use gtk4 as gtk;

use self::requests::{change_view_to_main_request, register_request};

use super::{
    widgets_creation::{
        build_application_window, create_center_button, create_entry, create_label_input_box,
        create_main_box,
    },
    MAIN_BOX_CSS,
};

use crate::{controller::controller_message::ControllerMessage, ADDRESS};

const LOGIN_BUTTON_TEXT: &str = "login";
const REALNAME_LABEL_TEXT: &str = "Your name:";
const NICKNAME_LABEL_TEXT: &str = "Nickname:";
const USERNAME_LABEL_TEXT: &str = "Username:";
const PASSWORD_LABEL_TEXT: &str = "Password:";
const ADDRESS_LABEL_TEXT: &str = "IP Address:";
pub struct RegisterView {
    pub realname_entry: Entry,
    pub nick_entry: Entry,
    pub username_entry: Entry,
    pub pass_entry: Entry,
    pub address_entry: Entry,
    pub login_button: Button,
    sender: Sender<ControllerMessage>,
}

impl RegisterView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            realname_entry: create_entry(""),
            nick_entry: create_entry(""),
            username_entry: create_entry(""),
            pass_entry: create_entry(""),
            address_entry: create_entry(""),
            login_button: create_center_button(LOGIN_BUTTON_TEXT),
            sender,
        }
    }

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

        let address_box = create_label_input_box(ADDRESS_LABEL_TEXT);
        address_box.append(&self.address_entry);
        main_box.append(&address_box);

        main_box.append(&self.login_button);

        self.connect_button(
            self.realname_entry.clone(),
            self.pass_entry.clone(),
            self.nick_entry.clone(),
            self.username_entry.clone(),
            self.address_entry.clone(),
            self.sender.clone(),
        );

        window.set_child(Some(&main_box));

        window
    }

    fn connect_button(
        &self,
        realname_entry: Entry,
        pass_entry: Entry,
        nick_entry: Entry,
        username_entry: Entry,
        address_entry: Entry,
        sender: Sender<ControllerMessage>,
    ) {
        self.login_button.connect_clicked(move |_| {
            let pass = pass_entry.text();
            let nickname = nick_entry.text();
            let username = username_entry.text();
            let realname = realname_entry.text();
            let address = Self::unpack_entry(address_entry.text());

            if Self::register_fiels_are_valid(&pass, &nickname, &username, &realname) {
                register_request(
                    pass,
                    nickname.clone(),
                    username,
                    realname,
                    address,
                    sender.clone(),
                );
                change_view_to_main_request(nickname, sender.clone());
            }
        });
    }

    fn register_fiels_are_valid(
        pass: &GString,
        nickname: &GString,
        username: &GString,
        realname: &GString,
    ) -> bool {
        !realname.is_empty() && !pass.is_empty() && !nickname.is_empty() && !username.is_empty()
    }

    fn unpack_entry(address: GString) -> String {
        if address.is_empty() {
            ADDRESS.to_string()
        } else {
            address.to_string()
        }
    }
}
