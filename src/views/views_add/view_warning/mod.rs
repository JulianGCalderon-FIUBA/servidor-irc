use gtk::{prelude::*, Align::Start, Application, ApplicationWindow, Button};
use gtk4 as gtk;

use crate::views::widgets_creation::{
    build_application_window, create_center_button, create_label,
};

use super::{
    widgets_creation::{create_main_box_add_view, create_title},
    CONTINUE_BUTTON_TEXT,
};

const TITLE: &str = "We are sorry :(";

pub struct WarningView {
    button: Button,
}

impl Default for WarningView {
    fn default() -> Self {
        Self::new()
    }
}

impl WarningView {
    pub fn new() -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
        }
    }

    pub fn get_view(&mut self, app: Application, warning_text: String) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        let title = create_title(TITLE);
        main_box.append(&title);

        let label = create_label(&warning_text);
        label.set_halign(Start);
        label.set_margin_start(20);
        main_box.append(&label);

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
