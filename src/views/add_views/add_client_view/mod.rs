/// Contains definition of used requests.
pub mod requests;

use gtk4::{
    glib::Sender,
    prelude::ComboBoxExtManual,
    traits::{BoxExt, ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Button, ComboBoxText,
};

use self::requests::add_client_button_request;

use super::{
    add_channel_view::widgets_creation::create_combobox,
    widgets_creation::{create_main_box_add_view, create_title},
};

use crate::{
    controller::controller_message::ControllerMessage,
    views::widgets_creation::{
        build_application_window, create_center_button, create_label_input_box,
    },
};

const ADD_CLIENT_BUTTON_TEXT: &str = "Add client";
const CLIENT_LABEL_TEXT: &str = "Client:";
const TITLE: &str = "Add client";

/// Shows add client view.  
/// Contains a client entry and an add new client button.  
/// Uses sender to communicate with controller.
pub struct AddClientView {
    add_client_button: Button,
    client_combobox: ComboBoxText,
    clients: Vec<String>,
    sender: Sender<ControllerMessage>,
}

impl AddClientView {
    /// Creates new [`AddClientView`]
    pub fn new(clients: Vec<String>, sender: Sender<ControllerMessage>) -> Self {
        Self {
            add_client_button: create_center_button(ADD_CLIENT_BUTTON_TEXT),
            client_combobox: create_combobox(),
            clients,
            sender,
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

        let client_box = create_label_input_box(CLIENT_LABEL_TEXT);
        self.refill_combobox();
        client_box.append(&self.client_combobox);
        main_box.append(&client_box);

        main_box.append(&self.add_client_button);

        self.connect_add_client_button(self.client_combobox.clone(), self.sender.clone());

        window.set_child(Some(&main_box));

        window
    }

    /// Connects add client button.
    ///
    /// Sends add client request to the controller.
    fn connect_add_client_button(&self, combobox: ComboBoxText, sender: Sender<ControllerMessage>) {
        self.add_client_button.connect_clicked(move |_| {
            if combobox.active_text().is_none() {
                return;
            }

            add_client_button_request(combobox.active_text().unwrap().to_string(), sender.clone());
        });
    }

    /// Fills combobox options with existing clients.
    fn refill_combobox(&mut self) {
        for client in &self.clients {
            self.client_combobox.append_text(&client.clone());
        }
        self.client_combobox.set_active(Some(0));
    }
}