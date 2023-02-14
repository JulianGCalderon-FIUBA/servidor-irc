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

/// Shows warning view.
/// Contains the warning label and an exit button.
pub struct WarningView {
    button: Button,
    warning_text: String,
}

impl WarningView {
    /// Creates new [`WarningView`]
    pub fn new(warning_text: String) -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
            warning_text,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        let title = create_title(TITLE);
        main_box.append(&title);

        let label = create_label(&self.warning_text);
        label.set_halign(Start);
        label.set_margin_start(20);
        main_box.append(&label);

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
