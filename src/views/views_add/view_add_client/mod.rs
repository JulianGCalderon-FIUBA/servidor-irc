pub mod requests;

use gtk::{glib::Sender, prelude::*, Application, ApplicationWindow, Button, Entry};
use gtk4 as gtk;

use self::requests::add_client_button_request;

use super::{
    super::{view_main::utils::entry_is_valid, widgets_creation::create_entry},
    widget_creations::{create_main_box_add_view, create_title},
};

use crate::{
    controller::controller_message::ControllerMessage,
    views::widgets_creation::{
        build_application_window, create_center_button, create_label_input_box,
    },
};

const TITLE: &str = "Add client";
const ADD_CLIENT_BUTTON_TEXT: &str = "Add client";
const CLIENT_LABEL_TEXT: &str = "Client:";

pub struct AddClientView {
    pub client_entry: Entry,
    pub add_client_button: Button,
    sender: Sender<ControllerMessage>,
}

impl AddClientView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            client_entry: create_entry(""),
            add_client_button: create_center_button(ADD_CLIENT_BUTTON_TEXT),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        let client_box = create_label_input_box(CLIENT_LABEL_TEXT);
        client_box.append(&self.client_entry);
        main_box.append(&client_box);

        main_box.append(&self.add_client_button);

        self.connect_add_client_button(self.client_entry.clone(), self.sender.clone());

        window.set_child(Some(&main_box));

        window
    }

    fn connect_add_client_button(&self, input: Entry, sender: Sender<ControllerMessage>) {
        self.add_client_button.connect_clicked(move |_| {
            if !entry_is_valid(&input.text()) {
                return;
            }

            add_client_button_request(input.text(), sender.clone());
        });
    }
}
