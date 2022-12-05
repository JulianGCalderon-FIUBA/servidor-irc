/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk::{prelude::*, Application, ApplicationWindow, Button, Orientation::Horizontal};
use gtk4 as gtk;

use crate::views::{
    views_add::view_notifications::widgets_creation::{
        create_box_container, create_notifications_scrollwindow,
    },
    widgets_creation::{
        build_application_window, create_center_button, create_label, create_separator,
    },
};

use super::{
    widgets_creation::{create_main_box_add_view, create_title},
    CONTINUE_BUTTON_TEXT,
};

const NOTIFICATIONS_BOX_CSS: &str = "notifications_container";
const TITLE: &str = "Notifications";

/// Shows notifications view.  
/// Contains the notifications and an exit button.  
pub struct NotificationsView {
    button: Button,
}

impl Default for NotificationsView {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationsView {
    /// Creates new [`NotificationsView`]
    pub fn new() -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
        }
    }

    /// Returns the view's window.
    /// 
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application, notifications: Vec<String>) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        let scrollwindow = create_notifications_scrollwindow();
        let container = create_box_container();
        for (i, notification) in notifications.iter().enumerate() {
            if i != 0 {
                let separator = create_separator(Horizontal);
                separator.set_hexpand(true);
                container.append(&separator);
            }
            let label = create_label(notification);
            container.append(&label);
        }
        scrollwindow.set_child(Some(&container));
        main_box.append(&scrollwindow);
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
