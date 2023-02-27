/// Contains definition of used requests.
pub mod requests;

use gtk4::{
    traits::{BoxExt, ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Button, glib::Sender,
};

use crate::{views::widgets_creation::{
    build_application_window, create_center_button,
}, controller::controller_message::ControllerMessage};

use self::requests::close_safe_conv_request;

use super::{
    widgets_creation::{create_main_box_add_view, create_title},
    CONTINUE_BUTTON_TEXT,
};

/// Shows warning view.
/// Contains the warning label and an exit button.
pub struct CloseSafeConvView {
    button: Button,
    sender: Sender<ControllerMessage>,
}

impl CloseSafeConvView {
    /// Creates new [`WarningView`]
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
            sender,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application, client: String) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        let message = format!("Hola");
        let title = create_title(&message);
        main_box.append(&title);

        main_box.append(&self.button);

        self.connect_button(window.clone(), client, self.sender.clone());

        window.set_child(Some(&main_box));
        window
    }

    /// Connects exit button.
    ///
    /// Closes the window.
    fn connect_button(&mut self, window: ApplicationWindow, client: String, sender: Sender<ControllerMessage>) {
        self.button.connect_clicked(move |_| {
            close_safe_conv_request(client.clone(), sender.clone());
            window.close();
        });
    }
}
