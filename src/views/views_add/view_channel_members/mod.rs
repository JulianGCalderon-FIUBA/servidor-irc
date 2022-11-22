use gtk::{ prelude::*, Align::Start, Application, ApplicationWindow, Button };
use gtk4 as gtk;

use crate::views::widgets_creation::{
    build_application_window,
    create_center_button,
    create_label,
};

use super::widget_creations::{ create_main_box_add_view, create_title };

pub struct ChannelMembersView {
    button: Button,
}

impl Default for ChannelMembersView {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelMembersView {
    pub fn new() -> Self {
        Self {
            button: create_center_button("ok"),
        }
    }

    pub fn get_view(&mut self, app: Application, clients: Vec<String>) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title("Miembros"));

        for client in &clients {
            //mejorar
            let label = create_label(client);
            label.set_halign(Start);
            label.set_margin_start(20);
            main_box.append(&label);
        }

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