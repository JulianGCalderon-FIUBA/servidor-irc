/// Contains definition of used requests.
pub mod request;

use self::request::invite_request;
use gtk4::{
    glib::Sender,
    prelude::ComboBoxExtManual,
    traits::{BoxExt, ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Button, ComboBoxText,
};

use super::add_channel_view::widgets_creation::create_combobox;
use super::widgets_creation::create_main_box_add_view;
use super::widgets_creation::create_title;

use crate::controller::controller_message::ControllerMessage;
use crate::views::widgets_creation::{
    build_application_window, create_center_button, create_label_input_box,
};

const CHANNEL_LABEL_TEXT: &str = "Channel:";
const INVITE_BUTTON_TEXT: &str = "Send invite";
const TITLE: &str = "Send invite";

/// Shows invite view.
/// Contains a channel entry and an invite button.
/// Uses sender to communicate with controller.
pub struct InviteView {
    channel_combobox: ComboBoxText,
    channels: Vec<String>,
    invite_button: Button,
    sender: Sender<ControllerMessage>,
}

impl InviteView {
    /// Creates new [`InviteView`]
    pub fn new(channels: Vec<String>, sender: Sender<ControllerMessage>) -> Self {
        Self {
            channel_combobox: create_combobox(),
            channels,
            invite_button: create_center_button(INVITE_BUTTON_TEXT),
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

        let channel_box = create_label_input_box(CHANNEL_LABEL_TEXT);
        self.refill_combobox();
        channel_box.append(&self.channel_combobox);
        main_box.append(&channel_box);

        main_box.append(&self.invite_button);

        self.connect_invite_button(self.channel_combobox.clone(), self.sender.clone());

        window.set_child(Some(&main_box));
        window
    }

    /// Connects invite button.
    ///
    /// Sends invite request to the controller.
    fn connect_invite_button(&self, combobox: ComboBoxText, sender: Sender<ControllerMessage>) {
        self.invite_button.connect_clicked(move |_| {
            if combobox.active_text().is_none() {
                return;
            }

            invite_request(combobox.active_text().unwrap().to_string(), sender.clone());
        });
    }

    /// Fills combobox options with existing clients.
    fn refill_combobox(&mut self) {
        for channel in &self.channels {
            self.channel_combobox.append_text(&channel.clone());
        }
        self.channel_combobox.set_active(Some(0));
    }
}
