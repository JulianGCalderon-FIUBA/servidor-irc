/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk::{prelude::*, Application, ApplicationWindow, Button};
use gtk4 as gtk;

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
}

impl Default for UserInfoView {
    fn default() -> Self {
        Self::new()
    }
}

impl UserInfoView {
    /// Creates new [`UserInfoView`]
    pub fn new() -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
        }
    }

    /// Returns the view's window.
    /// 
    /// Receives the controller's app.
    pub fn get_view(
        &mut self,
        app: Application,
        realname: String,
        servername: String,
        nickname: String,
        username: String,
    ) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        main_box.append(&create_user_info_label(&format!(
            "{REALNAME_LABEL_TEXT} {realname}"
        )));
        main_box.append(&create_user_info_label(&format!(
            "{NICKNAME_LABEL_TEXT} {nickname}"
        )));
        main_box.append(&create_user_info_label(&format!(
            "{SERVERNAME_LABEL_TEXT} {servername}"
        )));
        main_box.append(&create_user_info_label(&format!(
            "{USERNAME_LABEL_TEXT} {username}"
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
