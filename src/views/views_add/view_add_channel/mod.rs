pub mod widget_creations;

use gtk::{
    glib::Sender, prelude::*, Application, ApplicationWindow, Box, Button, ComboBoxText, Entry,
    Orientation::Horizontal, Orientation::Vertical,
};
use gtk4 as gtk;

use self::widget_creations::{
    create_active_button, create_box, create_combobox, create_inactive_button,
};

use super::widget_creations::create_main_box_add_view;
use super::{
    super::{view_main::utils::entry_is_valid, widgets_creation::create_entry},
    widget_creations::{create_add_channel_buton, create_label_box, create_title},
};

use crate::controller::controller_message::ControllerMessage;
use crate::views::APP_TITLE;

const TITLE: &str = "Add channel";
const JOIN_CHANNEL_BUTTON_TEXT: &str = "Join existing channel";
const CREATE_CHANNEL_BUTTON_TEXT: &str = "Create new channel";
const ADD_CHANNEL_BUTTON_TEXT: &str = "Add channel";
const CHANNEL_LABEL_TEXT: &str = "Channel:";
const ACTIVE_SELECT_BUTTON_CSS: &str = "active_select_button";
const INACTIVE_SELECT_BUTTON_CSS: &str = "inactive_select_button";
const DISABLE_SELECT_BUTTON_CSS: &str = "disable_select_button";
const ADD_CHANNEL_ENTRY_CSS: &str = "add_channel_entry";
const ERROR_TEXT: &str = "ERROR";

pub struct AddChannelView {
    pub join_channel_button: Button,
    pub create_channel_button: Button,
    pub join_channel_box: Box,
    pub create_channel_box: Box,
    pub channel_entry: Entry,
    pub channel_combobox: ComboBoxText,
    pub add_new_channel_button: Button,
    pub add_existing_channel_button: Button,
    sender: Sender<ControllerMessage>,
}

impl AddChannelView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            join_channel_button: create_active_button(JOIN_CHANNEL_BUTTON_TEXT),
            create_channel_button: create_inactive_button(CREATE_CHANNEL_BUTTON_TEXT),
            join_channel_box: create_box(Vertical),
            create_channel_box: create_box(Vertical),
            channel_entry: create_entry(""),
            channel_combobox: create_combobox(),
            add_new_channel_button: create_add_channel_buton(ADD_CHANNEL_BUTTON_TEXT),
            add_existing_channel_button: create_add_channel_buton(ADD_CHANNEL_BUTTON_TEXT),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application, channels: Vec<String>) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(&app)
            .title(APP_TITLE)
            .build();

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        let select_box = create_box(Horizontal);
        select_box.append(&self.join_channel_button);
        select_box.append(&self.create_channel_button);
        main_box.append(&select_box);

        self.append_join_channel_box(channels.clone(), main_box.clone());
        self.append_create_channel_box(main_box.clone());

        if channels.is_empty() {
            self.disable_join_channel_option();
        }

        self.connect_select_button(
            self.join_channel_button.clone(),
            self.create_channel_button.clone(),
            self.join_channel_box.clone(),
            self.create_channel_box.clone(),
        );
        self.connect_select_button(
            self.create_channel_button.clone(),
            self.join_channel_button.clone(),
            self.create_channel_box.clone(),
            self.join_channel_box.clone(),
        );

        self.connect_add_new_channel_button(self.channel_entry.clone(), self.sender.clone());
        if !channels.is_empty() {
            self.connect_add_existing_channel_button(
                self.channel_combobox.clone(),
                self.sender.clone(),
            );
        }

        window.set_child(Some(&main_box));
        window
    }

    fn append_join_channel_box(&mut self, channels: Vec<String>, main_box: Box) {
        let entry_box = create_label_box(CHANNEL_LABEL_TEXT);
        self.refill_combobox(channels);
        entry_box.append(&self.channel_combobox);
        self.join_channel_box.append(&entry_box);
        self.join_channel_box
            .append(&self.add_existing_channel_button);
        main_box.append(&self.join_channel_box);
    }

    fn append_create_channel_box(&mut self, main_box: Box) {
        let entry_box = create_label_box(CHANNEL_LABEL_TEXT);
        self.channel_entry.add_css_class(ADD_CHANNEL_ENTRY_CSS);
        entry_box.append(&self.channel_entry);
        self.create_channel_box.append(&entry_box);
        self.create_channel_box.set_visible(false);
        self.create_channel_box.append(&self.add_new_channel_button);
        main_box.append(&self.create_channel_box);
    }

    fn refill_combobox(&mut self, channels: Vec<String>) {
        for channel in &channels {
            self.channel_combobox.append_text(&channel.clone());
        }
    }

    fn disable_join_channel_option(&mut self) {
        self.join_channel_button.set_sensitive(false);
        Self::active_button(self.create_channel_button.clone());
        Self::disable_button(self.join_channel_button.clone());
        Self::switch_visibility(
            self.create_channel_box.clone(),
            self.join_channel_box.clone(),
        );
    }

    fn connect_select_button(
        &self,
        active_button: Button,
        disactive_button: Button,
        visible_box: Box,
        no_visible_box: Box,
    ) {
        let create_channel_button_clone = active_button.clone();
        active_button.connect_clicked(move |_| {
            Self::active_button(create_channel_button_clone.clone());
            Self::disactive_button(disactive_button.clone());
            Self::switch_visibility(visible_box.clone(), no_visible_box.clone());
        });
    }

    fn connect_add_existing_channel_button(
        &self,
        combobox: ComboBoxText,
        sender: Sender<ControllerMessage>,
    ) {
        self.add_existing_channel_button.connect_clicked(move |_| {
            if combobox.active_text().is_none() {
                return;
            }

            let add_channel = ControllerMessage::AddNewChannel {
                channel: combobox.active_text().unwrap(),
            };
            sender.send(add_channel).expect(ERROR_TEXT);
        });
    }

    fn connect_add_new_channel_button(&self, input: Entry, sender: Sender<ControllerMessage>) {
        self.add_new_channel_button.connect_clicked(move |_| {
            if !entry_is_valid(&input.text()) {
                return;
            }

            let add_channel = ControllerMessage::AddNewChannel {
                channel: input.text(),
            };
            sender.send(add_channel).expect(ERROR_TEXT);
        });
    }

    fn active_button(button: Button) {
        button.remove_css_class(INACTIVE_SELECT_BUTTON_CSS);
        button.add_css_class(ACTIVE_SELECT_BUTTON_CSS);
    }

    fn disactive_button(button: Button) {
        button.remove_css_class(ACTIVE_SELECT_BUTTON_CSS);
        button.add_css_class(INACTIVE_SELECT_BUTTON_CSS);
    }

    fn disable_button(button: Button) {
        button.remove_css_class(ACTIVE_SELECT_BUTTON_CSS);
        button.add_css_class(DISABLE_SELECT_BUTTON_CSS);
    }

    fn switch_visibility(visible_box: Box, no_visible_box: Box) {
        visible_box.set_visible(true);
        no_visible_box.set_visible(false);
    }
}
