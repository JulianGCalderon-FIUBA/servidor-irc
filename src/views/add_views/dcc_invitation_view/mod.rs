/// Contains definition of used requests.
pub mod requests;

use std::net::SocketAddr;

use gtk4::{
    glib::Sender,
    traits::{BoxExt, ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Button,
    Orientation::{Horizontal, Vertical},
};

use self::requests::{accept_request, decline_request};

use crate::views::widgets_creation::{
    build_application_window, create_center_button, create_label, create_main_box,
};

use crate::controller::controller_message::ControllerMessage;

const ACCEPT_BUTTON_TEXT: &str = "Accept";
const DECLINE_BUTTON_TEXT: &str = "Decline";
const INVITATION: &str = "wants to have a safe conversation with you";

/// Shows ip selection view.
/// Contains an address entry.
/// Uses sender to communicate with controller.
pub struct DccInvitationView {
    accept_button: Button,
    decline_button: Button,
    sender: Sender<ControllerMessage>,
}

impl DccInvitationView {
    /// Creates new [`DccInvitationView`].
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            accept_button: create_center_button(ACCEPT_BUTTON_TEXT),
            decline_button: create_center_button(DECLINE_BUTTON_TEXT),
            sender,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(
        &mut self,
        app: Application,
        client: String,
        address: SocketAddr,
    ) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box(Vertical, 300, 300);

        let message = format!("{client} {INVITATION}");
        let invitation = create_label(&message);

        main_box.append(&invitation);

        let button_box = create_main_box(Horizontal, 150, 300);
        self.connect_accept_button(client.clone(), address, self.sender.clone());
        self.connect_decline_button(client, self.sender.clone());

        button_box.append(&self.accept_button);
        button_box.append(&self.decline_button);
        main_box.append(&button_box);

        // self.connect_button(self.address_entry.clone(), self.sender.clone());

        window.set_child(Some(&main_box));

        window
    }

    /// Connects accept button.
    ///
    /// Sends accept request to the controller.
    fn connect_accept_button(
        &self,
        client: String,
        address: SocketAddr,
        sender: Sender<ControllerMessage>,
    ) {
        self.accept_button.connect_clicked(move |_| {
            accept_request(client.clone(), address, sender.clone());
        });
    }

    /// Connects decline button.
    ///
    /// Sends decline request to the controller.
    fn connect_decline_button(&self, client: String, sender: Sender<ControllerMessage>) {
        self.decline_button.connect_clicked(move |_| {
            decline_request(client.clone(), sender.clone());
        });
    }
}
