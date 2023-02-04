/// Contains definition of used requests.
pub mod requests;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk::{glib::Sender, prelude::*, Align::Start, Application, ApplicationWindow, Button, Label};
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::{
        add_views::channel_members_view::widgets_creation::create_kick_label_box,
        widgets_creation::{build_application_window, create_center_button, create_label},
    },
};

use self::{
    requests::kick_request,
    widgets_creation::{create_kick_button, create_kick_label},
};

use super::{
    widgets_creation::{create_main_box_add_view, create_title},
    CONTINUE_BUTTON_TEXT,
};

const OPERATOR_FIRST_CHARACTER: &str = "@";
const TITLE: &str = "Members";

/// Shows channel members view.
/// Contains an exit button.
/// Uses sender to communicate with controller.
pub struct SafeConversationView {
    button: Button,
    sender: Sender<ControllerMessage>,
}

impl SafeConversationView {
    /// Creates new [`SafeConversationView`]
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
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
