pub mod request;

use gtk::ComboBoxText;
use gtk::{ glib::Sender, prelude::*, Application, ApplicationWindow, Button, Entry };
use gtk4 as gtk;

use self::request::invite_request;

use super::view_add_channel::widget_creations::create_combobox;
use super::widget_creations::create_main_box_add_view;
use super::{ super::{ view_main::utils::entry_is_valid }, widget_creations::create_title };

use crate::controller::controller_message::ControllerMessage;
use crate::views::widgets_creation::{
    build_application_window,
    create_center_button,
    create_label_input_box,
};

pub struct InviteView {
    pub channel_combobox: ComboBoxText,
    pub invite_button: Button,
    sender: Sender<ControllerMessage>,
}

impl InviteView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            channel_combobox: create_combobox(),
            invite_button: create_center_button("Send invite"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application, channels: Vec<String>) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        let title = create_title("Send Invite");
        main_box.append(&title);

        let channel_box = create_label_input_box("Channel:");
        self.refill_combobox(channels);
        channel_box.append(&self.channel_combobox);
        main_box.append(&channel_box);

        main_box.append(&self.invite_button);

        self.connect_invite_button(self.channel_combobox.clone(), self.sender.clone());

        window.set_child(Some(&main_box));
        window
    }

    fn connect_invite_button(&self, combobox: ComboBoxText, sender: Sender<ControllerMessage>) {
        self.invite_button.connect_clicked(move |_| {
            if combobox.active_text().is_none() {
                return;
            }

            invite_request(combobox.active_text().unwrap(), sender.clone());
        });
    }

    fn refill_combobox(&mut self, channels: Vec<String>) {
        for channel in &channels {
            self.channel_combobox.append_text(&channel.clone());
        }
    }
}