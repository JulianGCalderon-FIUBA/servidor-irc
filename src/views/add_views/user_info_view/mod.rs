/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk4::{
    traits::{BoxExt, ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Button,
};

use crate::views::{
    widgets_creation::{build_application_window, create_center_button},
    NICKNAME_LABEL_TEXT, REALNAME_LABEL_TEXT, SERVERNAME_LABEL_TEXT, USERNAME_LABEL_TEXT,
};

use self::widgets_creation::create_user_info_label;

use super::{
    widgets_creation::{create_main_box_add_view, create_title},
    CONTINUE_BUTTON_TEXT,
};

const TITLE: &str = "User Information";

/// Shows user info view.
/// Contains the user info and an exit button.
pub struct UserInfoView {
    button: Button,
    nickname: String,
    realname: String,
    servername: String,
    username: String,
}

impl UserInfoView {
    /// Creates new [`UserInfoView`]
    pub fn new(nickname: String, realname: String, servername: String, username: String) -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
            nickname,
            realname,
            servername,
            username,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        main_box.append(&create_user_info_label(&format!(
            "{REALNAME_LABEL_TEXT} {}",
            self.realname
        )));
        main_box.append(&create_user_info_label(&format!(
            "{NICKNAME_LABEL_TEXT} {}",
            self.nickname
        )));
        main_box.append(&create_user_info_label(&format!(
            "{SERVERNAME_LABEL_TEXT} {}",
            self.servername
        )));
        main_box.append(&create_user_info_label(&format!(
            "{USERNAME_LABEL_TEXT} {}",
            self.username
        )));

        main_box.append(&self.button);

        self.connect_button(window.clone());

        window.set_child(Some(&main_box));
        window
    }

    /// Connects exit button.
    ///
    /// Closes the window.
    fn connect_button(&mut self, window: ApplicationWindow) {
        self.button.connect_clicked(move |_| {
            window.close();
        });
    }
}
