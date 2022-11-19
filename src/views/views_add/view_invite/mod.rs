use gtk::{glib::Sender, prelude::*, Application, ApplicationWindow, Button, Entry};
use gtk4 as gtk;

use super::widget_creations::create_main_box_add_view;
use super::{
    super::{view_main::utils::entry_is_valid, widgets_creation::create_entry},
    widget_creations::{create_add_channel_buton, create_label_box, create_title},
};

use crate::controller::controller_message::ControllerMessage;
use crate::views::{APP_TITLE, ERROR_TEXT};

pub struct InviteView {
    pub channel_entry: Entry,
    pub invite_button: Button,
    sender: Sender<ControllerMessage>,
}

impl InviteView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            channel_entry: create_entry(""),
            invite_button: create_add_channel_buton("Send invite"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(&app)
            .title(APP_TITLE)
            .build();

        let main_box = create_main_box_add_view();

        let title = create_title("Send Invite");
        main_box.append(&title);

        let channel_box = create_label_box("Channel:");
        channel_box.append(&self.channel_entry);
        main_box.append(&channel_box);

        main_box.append(&self.invite_button);

        self.connect_invite_button(self.channel_entry.clone(), self.sender.clone());

        window.set_child(Some(&main_box));
        window
    }

    fn connect_invite_button(&self, input: Entry, sender: Sender<ControllerMessage>) {
        self.invite_button.connect_clicked(move |_| {
            if !entry_is_valid(&input.text()) {
                return;
            }

            let invite = ControllerMessage::SendInviteMessage {
                channel: input.text(),
            };
            sender.send(invite).expect(ERROR_TEXT);
        });
    }
}
