pub mod widgets_creation;

use gtk::{prelude::*, Application, ApplicationWindow, Button};
use gtk4 as gtk;

use crate::views::widgets_creation::{build_application_window, create_center_button};

use self::widgets_creation::create_user_info_label;

use super::widgets_creation::{create_main_box_add_view, create_title};

pub struct UserInfoView {
    button: Button,
}

impl Default for UserInfoView {
    fn default() -> Self {
        Self::new()
    }
}

impl UserInfoView {
    pub fn new() -> Self {
        Self {
            button: create_center_button("ok"),
        }
    }

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

        main_box.append(&create_title("User Information"));

        main_box.append(&create_user_info_label(&format!("Your name: {realname}")));
        main_box.append(&create_user_info_label(&format!("Nickname: {nickname}")));
        main_box.append(&create_user_info_label(&format!(
            "Servername: {servername}"
        )));
        main_box.append(&create_user_info_label(&format!("Username: {username}")));

        main_box.append(&self.button);

        self.connect_button(window.clone());

        window.set_child(Some(&main_box));
        window
    }

    fn connect_button(&mut self, window: ApplicationWindow) {
        self.button.connect_clicked(move |_| {
            window.close();
        });
    }
}
