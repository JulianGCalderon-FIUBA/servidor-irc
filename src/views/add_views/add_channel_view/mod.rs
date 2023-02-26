/// Contains definition of used requests.
pub mod requests;

/// Contains useful functions.
pub mod utils;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk::prelude::ComboBoxExtManual;
use gtk::Label;
use gtk::{
    glib::Sender, Application, ApplicationWindow, Box, Button, ComboBoxText, Entry,
    Orientation::Horizontal, Orientation::Vertical,
};
use gtk4 as gtk;
use gtk4::{
    prelude::EditableExt,
    traits::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt},
};

use self::requests::join_channel_request;
use self::utils::{activate_button, disable_button, disactivate_button, switch_visibility};
use self::widgets_creation::{
    create_active_button, create_box, create_combobox, create_inactive_button,
};

use super::widgets_creation::create_main_box_add_view;
use super::{
    super::{main_view::utils::entry_is_valid, widgets_creation::create_entry},
    widgets_creation::create_title,
};

use crate::controller::controller_message::ControllerMessage;
use crate::controller::utils::{is_channel, is_not_empty, vec_is_not_empty};
use crate::views::widgets_creation::{
    build_application_window, create_center_button, create_error_label, create_label,
    create_label_input_box,
};

const ADD_CHANNEL_BUTTON_TEXT: &str = "Add channel";
const CHANNEL_FIRST_CHARACTER: &str = "#";
const CHANNEL_LABEL_TEXT: &str = "Channel:";
const CHANNEL_NAME_MAX_CHARACTERS: usize = 15;
const CREATE_CHANNEL_BUTTON_TEXT: &str = "Create new channel";
const ERR_CHANNEL_NAME_EMPTY: &str = "¡Channel name is empty!";
const ERR_CHANNEL_NAME_TOO_LONG: &str = "¡Channel name too long!";
const JOIN_CHANNEL_BUTTON_TEXT: &str = "Join existing channel";
const TITLE: &str = "Add channel";
const CANT_JOIN_CHANNEL_TOOLTIP: &str = "There are no channels available to be joined";

const ACTIVE_SELECT_BUTTON_CSS: &str = "active_select_button";
const ADD_CHANNEL_ENTRY_CSS: &str = "add_channel_entry";
const CHANNEL_FIRST_CHARACTER_LABEL_CSS: &str = "channel_first_character";
const DISABLE_SELECT_BUTTON_CSS: &str = "disable_select_button";
const INACTIVE_SELECT_BUTTON_CSS: &str = "inactive_select_button";

/// Shows add channel view.
/// Contains a channel entry and an add new channel button.
/// Uses sender to communicate with controller.
pub struct AddChannelView {
    add_existing_channel_button: Button,
    add_new_channel_button: Button,
    channel_combobox: ComboBoxText,
    channel_entry: Entry,
    channels: Vec<String>,
    create_channel_box: Box,
    create_channel_button: Button,
    error_label: Label,
    join_channel_box: Box,
    join_channel_button: Button,
    sender: Sender<ControllerMessage>,
}

impl AddChannelView {
    /// Creates new [`AddChannelView`]
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            add_existing_channel_button: create_center_button(ADD_CHANNEL_BUTTON_TEXT),
            add_new_channel_button: create_center_button(ADD_CHANNEL_BUTTON_TEXT),
            channel_combobox: create_combobox(),
            channel_entry: create_entry(""),
            channels: vec![],
            create_channel_box: create_box(Vertical),
            create_channel_button: create_inactive_button(CREATE_CHANNEL_BUTTON_TEXT),
            error_label: create_error_label(),
            join_channel_box: create_box(Vertical),
            join_channel_button: create_active_button(JOIN_CHANNEL_BUTTON_TEXT),
            sender,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application, channels: Vec<String>) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        self.channels = channels;

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        let select_box = create_box(Horizontal);
        select_box.append(&self.join_channel_button);
        select_box.append(&self.create_channel_button);
        main_box.append(&select_box);

        self.append_join_channel_box(main_box.clone());
        self.append_create_channel_box(main_box.clone());

        if self.channels.is_empty() {
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

        self.connect_add_new_channel_button(
            self.channel_entry.clone(),
            self.error_label.clone(),
            self.sender.clone(),
        );
        if vec_is_not_empty(&self.channels) {
            self.connect_add_existing_channel_button(
                self.channel_combobox.clone(),
                self.sender.clone(),
            );
        }

        window.set_child(Some(&main_box));
        window
    }

    /// Add join existing channel box.
    ///
    /// Button is visible if user can join an existing channel.
    fn append_join_channel_box(&mut self, main_box: Box) {
        let entry_box = create_label_input_box(CHANNEL_LABEL_TEXT);
        self.refill_combobox();
        entry_box.append(&self.channel_combobox);
        self.join_channel_box.append(&entry_box);
        self.join_channel_box
            .append(&self.add_existing_channel_button);
        main_box.append(&self.join_channel_box);
    }

    /// Add join new channel box.
    ///
    /// Button is visible if user cannot join an existing channel.
    fn append_create_channel_box(&mut self, main_box: Box) {
        let entry_box = create_label_input_box(CHANNEL_LABEL_TEXT);
        self.channel_entry.add_css_class(ADD_CHANNEL_ENTRY_CSS);
        let channel_first_character = create_label(CHANNEL_FIRST_CHARACTER);
        channel_first_character.add_css_class(CHANNEL_FIRST_CHARACTER_LABEL_CSS);
        entry_box.append(&channel_first_character);
        entry_box.append(&self.channel_entry);
        self.create_channel_box.append(&entry_box);
        self.create_channel_box.append(&self.add_new_channel_button);
        self.create_channel_box.append(&self.error_label);
        self.create_channel_box.set_visible(false);
        main_box.append(&self.create_channel_box);
    }

    /// Fills combobox options with existing channels
    fn refill_combobox(&mut self) {
        for channel in &self.channels {
            self.channel_combobox.append_text(&channel.clone());
        }
        self.channel_combobox.set_active(Some(0));
    }

    /// Disables the join existing channel button.
    fn disable_join_channel_option(&mut self) {
        activate_button(self.create_channel_button.clone());
        disable_button(self.join_channel_button.clone(), CANT_JOIN_CHANNEL_TOOLTIP);
        switch_visibility(
            self.create_channel_box.clone(),
            self.join_channel_box.clone(),
        );
    }

    /// Connects select button.
    ///
    /// Changes visibility of joining option.
    fn connect_select_button(
        &self,
        active_button: Button,
        disactive_button: Button,
        visible_box: Box,
        no_visible_box: Box,
    ) {
        let create_channel_button_clone = active_button.clone();
        active_button.connect_clicked(move |_| {
            activate_button(create_channel_button_clone.clone());
            disactivate_button(disactive_button.clone());
            switch_visibility(visible_box.clone(), no_visible_box.clone());
        });
    }

    /// Connects add existing channel button.
    ///
    /// Sends join channel request to the controller.
    fn connect_add_existing_channel_button(
        &self,
        combobox: ComboBoxText,
        sender: Sender<ControllerMessage>,
    ) {
        self.add_existing_channel_button.connect_clicked(move |_| {
            if combobox.active_text().is_none() {
                return;
            }

            join_channel_request(combobox.active_text().unwrap().to_string(), sender.clone());
        });
    }

    /// Connects add new channel button.
    ///
    /// Sends join channel request to the controller.
    fn connect_add_new_channel_button(
        &self,
        input: Entry,
        error_label: Label,
        sender: Sender<ControllerMessage>,
    ) {
        self.add_new_channel_button.connect_clicked(move |_| {
            let mut text = input.text().to_string();
            error_label.set_text("");

            if !entry_is_valid(&text, CHANNEL_NAME_MAX_CHARACTERS) {
                if is_not_empty(&text) {
                    error_label.set_text(ERR_CHANNEL_NAME_TOO_LONG);
                } else {
                    error_label.set_text(&format!(
                        "{ERR_CHANNEL_NAME_EMPTY} Max: {CHANNEL_NAME_MAX_CHARACTERS} characters"
                    ));
                }
                return;
            }
            if !is_channel(&text) {
                text = format!("{CHANNEL_FIRST_CHARACTER}{text}");
            }

            join_channel_request(text, sender.clone());
        });
    }

    pub fn set_error_text(&mut self, text: String) {
        self.error_label.set_text(&text);
    }
}
