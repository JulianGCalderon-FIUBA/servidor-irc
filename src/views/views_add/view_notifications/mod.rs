pub mod widgets_creation;

use gtk::{prelude::*, Application, ApplicationWindow, Button};
use gtk4 as gtk;

use crate::views::{
    views_add::view_notifications::widgets_creation::{
        create_box_container, create_notifications_scrollwindow,
    },
    widgets_creation::{
        build_application_window, create_center_button, create_label, create_separator,
    },
};

use super::widget_creations::{create_main_box_add_view, create_title};

pub struct NotificationsView {
    button: Button,
}

impl Default for NotificationsView {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationsView {
    pub fn new() -> Self {
        Self {
            button: create_center_button("ok"),
        }
    }

    pub fn get_view(&mut self, app: Application, notifications: Vec<String>) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title("Notifications"));

        let scrollwindow = create_notifications_scrollwindow();
        let container = create_box_container();
        for (i, notification) in notifications.iter().enumerate() {
            if i != 0 {
                let separator = create_separator(gtk::Orientation::Horizontal);
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

    fn connect_button(&mut self, window: ApplicationWindow) {
        self.button.connect_clicked(move |_| {
            window.close();
        });
    }
}
