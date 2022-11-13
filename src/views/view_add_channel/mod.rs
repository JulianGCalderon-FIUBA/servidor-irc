mod widget_creations;

use gtk::{glib::Sender, prelude::*, Application, ApplicationWindow, Button, Entry, Orientation};
use gtk4 as gtk;

use self::widget_creations::{create_add_channel_buton, create_label_box};

use super::{widgets_creation::{create_entry, create_main_box}, view_main::utils::entry_is_valid};

use crate::controller::controller_message::ControllerMessage;

pub struct AddChannelView {
    pub channel_entry: Entry,
    pub add_channel_button: Button,
    sender: Sender<ControllerMessage>,
}

impl AddChannelView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            channel_entry: create_entry(""),
            add_channel_button: create_add_channel_buton("add channel"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(&app)
            .title("Lemon Pie IRC")
            .build();

        let main_box = create_main_box(Orientation::Vertical, 300, 300);
        main_box.add_css_class("main_box");

        let channel_box = create_label_box("Channel:");
        channel_box.append(&self.channel_entry);
        main_box.append(&channel_box);

        main_box.append(&self.add_channel_button);

        self.connect_add_channel_button(
            self.channel_entry.clone(),
            self.sender.clone(),
        );

        window.set_child(Some(&main_box));

        window
    }

    fn connect_add_channel_button(
        &self,
        input: Entry,
        sender: Sender<ControllerMessage>,
    ) {
        self.add_channel_button.connect_clicked(move |_| {
            if !entry_is_valid(&input.text()) {
                return;
            }
            
            let add_channel = ControllerMessage::AddNewChannel {
                channel: input.text(),
            };
            sender.send(add_channel).expect("ERROR");
        });
    }
}
