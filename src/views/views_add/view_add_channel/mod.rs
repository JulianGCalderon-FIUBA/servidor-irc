pub mod widget_creations;

use gtk::{ ComboBoxText, ComboBox };
use gtk::glib::GString;
use gtk::{
    glib::Sender,
    prelude::*,
    Application,
    ApplicationWindow,
    Button,
    Entry,
    Box,
    Orientation::Horizontal,
};
use gtk4 as gtk;

use self::widget_creations::{ create_box, create_active_button, create_disable_button };

use super::widget_creations::create_main_box_add_view;
use super::{
    super::{ view_main::utils::entry_is_valid, widgets_creation::create_entry },
    widget_creations::{ create_add_channel_buton, create_label_box, create_title },
};

use crate::controller::controller_message::ControllerMessage;
use crate::views::view_main::widgets_creation::create_button;

pub struct AddChannelView {
    pub join_channel_button: Button,
    pub create_channel_button: Button,
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
            join_channel_button: create_button(""),
            create_channel_button: create_button(""),
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

        main_box.append(&create_title("Add channel"));

        let select_box = create_box(Horizontal);
        self.join_channel_button = create_active_button("Join existing channel");
        select_box.append(&self.join_channel_button);
        self.create_channel_button = create_disable_button("Create new channel");
        select_box.append(&self.create_channel_button);
        main_box.append(&select_box);

        self.join_channel_box = create_label_box("Channel:");
        let combobox = ComboBoxText::builder().width_request(172).build();
        for channel in &channels {
            combobox.append_text(&channel.clone());
        }
        self.join_channel_box.append(&combobox);
        main_box.append(&self.join_channel_box);

        self.create_channel_box = create_label_box("Channel:");
        self.channel_entry.add_css_class("add_channel_entry");
        self.create_channel_box.append(&self.channel_entry);
        self.create_channel_box.set_visible(false);
        main_box.append(&self.create_channel_box);

        main_box.append(&self.add_channel_button);

        if channels.len() == 0 {
            self.join_channel_button.set_sensitive(false);
            self.create_channel_button.remove_css_class("inactive_select_button");
            self.create_channel_button.add_css_class("active_select_button");

            self.join_channel_button.remove_css_class("active_select_button");
            self.join_channel_button.add_css_class("disable_select_button");

            self.join_channel_box.set_visible(false);
            self.create_channel_box.set_visible(true);
        }

        self.connect_join_channel_button(
            self.join_channel_button.clone(),
            self.create_channel_button.clone(),
            self.join_channel_box.clone(),
            self.create_channel_box.clone()
        );
        self.connect_create_channel_button(
            self.join_channel_button.clone(),
            self.create_channel_button.clone(),
            self.join_channel_box.clone(),
            self.create_channel_box.clone()
        );

        self.connect_add_new_channel_button(self.channel_entry.clone(), self.sender.clone());
        if channels.len() > 0 {
            self.connect_add_existing_channel_button(combobox.clone(), self.sender.clone());
        }

        window.set_child(Some(&main_box));
        window
    }

    fn connect_join_channel_button(
        &self,
        join_channel_button: Button,
        create_channel_button: Button,
        join_channel_box: Box,
        create_channel_box: Box
    ) {
        let join_channel_button_clone = join_channel_button.clone();
        join_channel_button.connect_clicked(move |_| {
            join_channel_button_clone.remove_css_class("inactive_select_button");
            join_channel_button_clone.add_css_class("active_select_button");
            create_channel_button.remove_css_class("active_select_button");
            create_channel_button.add_css_class("inactive_select_button");

            join_channel_box.set_visible(true);
            create_channel_box.set_visible(false);
        });
    }

    fn connect_create_channel_button(
        &self,
        join_channel_button: Button,
        create_channel_button: Button,
        join_channel_box: Box,
        create_channel_box: Box
    ) {
        let create_channel_button_clone = create_channel_button.clone();
        create_channel_button.connect_clicked(move |_| {
            create_channel_button_clone.remove_css_class("inactive_select_button");
            create_channel_button_clone.add_css_class("active_select_button");

            join_channel_button.remove_css_class("active_select_button");
            join_channel_button.add_css_class("inactive_select_button");

            join_channel_box.set_visible(false);
            create_channel_box.set_visible(true);
        });
    }

    fn connect_add_existing_channel_button(
        &self,
        combobox: ComboBoxText,
        sender: Sender<ControllerMessage>
    ) {
        self.add_channel_button.connect_clicked(move |_| {
            if !entry_is_valid(&combobox.active_text().unwrap()) {
                return;
            }

            let add_channel = ControllerMessage::AddNewChannel {
                channel: combobox.active_text().unwrap().clone(),
            };
            sender.send(add_channel).expect("ERROR");
        });
    }

    fn connect_add_new_channel_button(&self, input: Entry, sender: Sender<ControllerMessage>) {
        self.add_channel_button.connect_clicked(move |_| {
            if !entry_is_valid(&input.text()) {
                return;
            }

            let add_channel = ControllerMessage::AddNewChannel {
                channel: input.text().clone(),
            };
            sender.send(add_channel).expect("ERROR");
        });
    }
}