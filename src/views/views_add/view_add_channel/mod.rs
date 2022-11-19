use gtk::Orientation;
use gtk::{ glib::Sender, prelude::*, Application, ApplicationWindow, Button, Entry, Box };
use gtk4 as gtk;

use super::widget_creations::{ create_main_box_add_view, create_label };
use super::{
    super::{ view_main::utils::entry_is_valid, widgets_creation::create_entry },
    widget_creations::{ create_add_channel_buton, create_label_box, create_title },
};

use crate::controller::controller_message::ControllerMessage;
use crate::views::view_main::widgets_creation::create_button;

pub struct AddChannelView {
    pub join_channel_box: Box,
    pub create_channel_box: Box,
    pub existing_channels: Vec<Button>,
    pub channel_entry: Entry,
    pub add_channel_button: Button,
    sender: Sender<ControllerMessage>,
}

impl AddChannelView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            join_channel_box: create_label_box(""),
            create_channel_box: create_label_box(""),
            existing_channels: vec![],
            channel_entry: create_entry(""),
            add_channel_button: create_add_channel_buton("add channel"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application, channels: Vec<String>) -> ApplicationWindow {
        let window = ApplicationWindow::builder().application(&app).title("Lemon Pie IRC").build();

        let main_box = create_main_box_add_view();

        let title = create_title("Add channel");
        main_box.append(&title);

        let select_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .build();
        let join_channel_button = Button::builder().label("Unirse a canal existente").build();
        select_box.append(&join_channel_button);
        let create_channel_button = Button::builder().label("Crear nuevo canal").build();
        select_box.append(&create_channel_button);
        main_box.append(&select_box);

        self.join_channel_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .build();
        for channel in channels {
            let button = create_button(&channel);
            self.join_channel_box.append(&button);
            self.existing_channels.push(button);
        }
        main_box.append(&self.join_channel_box);

        self.create_channel_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .build();
        let channel_box = create_label_box("Channel:");
        channel_box.append(&self.channel_entry);
        self.create_channel_box.append(&channel_box);
        self.create_channel_box.append(&self.add_channel_button);
        main_box.append(&self.create_channel_box);

        self.connect_join_channel_button(
            join_channel_button.clone(),
            self.join_channel_box.clone(),
            self.create_channel_box.clone()
            
        );
        self.connect_create_channel_button(
            create_channel_button.clone(),
            self.join_channel_box.clone(),
            self.create_channel_box.clone()
        );
        self.connect_add_channel_button(self.channel_entry.clone(), self.sender.clone());

        window.set_child(Some(&main_box));
        window
    }

    fn connect_join_channel_button(&self, button: Button, join_channel_box: Box, create_channel_box: Box) {
        button.connect_clicked(move |_| {
            join_channel_box.set_visible(true);
            create_channel_box.set_visible(false);
        });
    }

    fn connect_create_channel_button(&self, button: Button, join_channel_box: Box, create_channel_box: Box) {
        button.connect_clicked(move |_| {
            join_channel_box.set_visible(false);
            create_channel_box.set_visible(true);
        });
    }

    fn connect_add_channel_button(&self, input: Entry, sender: Sender<ControllerMessage>) {
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