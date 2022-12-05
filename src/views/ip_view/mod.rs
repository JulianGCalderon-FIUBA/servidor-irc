pub mod requests;

use gtk::{
    glib::{GString, Sender},
    prelude::*,
    Application, ApplicationWindow, Button, Entry, Orientation,
};
use gtk4 as gtk;

use self::requests::to_register_request;

use super::{
    widgets_creation::{
        build_application_window, create_center_button, create_entry, create_label_input_box,
        create_main_box, create_label,
    },
    MAIN_BOX_CSS,
};

use crate::{controller::controller_message::ControllerMessage, ADDRESS};

const CONNECT_BUTTON_TEXT: &str = "Connect";
const ADDRESS_LABEL_TEXT: &str = "IP Address:";
const ADDRESS_MESSAGE: &str = "Leave it empty to use the default IP...";
pub struct IpView {
    pub address_entry: Entry,
    pub ok_button: Button,
    sender: Sender<ControllerMessage>,
}

impl IpView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            address_entry: create_entry(""),
            ok_button: create_center_button(CONNECT_BUTTON_TEXT),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box(Orientation::Vertical, 150, 300);
        main_box.add_css_class(MAIN_BOX_CSS);

        let address_box = create_label_input_box(ADDRESS_LABEL_TEXT);
        address_box.append(&self.address_entry);
        main_box.append(&address_box);

        let label = create_label(ADDRESS_MESSAGE);
        main_box.append(&label);

        main_box.append(&self.ok_button);

        self.connect_button(
            self.address_entry.clone(),
            self.sender.clone(),
        );

        window.set_child(Some(&main_box));

        window
    }

    fn connect_button(
        &self,
        address_entry: Entry,
        sender: Sender<ControllerMessage>,
    ) {
        self.ok_button.connect_clicked(move |_| {
            let address = Self::unpack_entry(address_entry.text());

            if Self::register_fiels_are_valid(&address) {
                to_register_request(
                    address,
                    sender.clone(),
                );
                // change_view_to_main_request(nickname, sender.clone());
            }
        });
    }

    fn register_fiels_are_valid(
        address: &String,
    ) -> bool {
        true
    }

    fn unpack_entry(address: GString) -> String {
        if address.is_empty() {
            ADDRESS.to_string()
        } else {
            address.to_string()
        }
    }
}
