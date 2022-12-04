use gtk::{prelude::*, Box, Button, ComboBoxText, Orientation};
use gtk4 as gtk;

use crate::views::widgets_creation::create_button;

use super::{ACTIVE_SELECT_BUTTON_CSS, INACTIVE_SELECT_BUTTON_CSS};

pub fn create_box(orientation: Orientation) -> Box {
    Box::builder()
        .orientation(orientation)
        .margin_top(20)
        .margin_bottom(20)
        .halign(gtk::Align::Center)
        .build()
}

pub fn create_active_button(label: &str) -> Button {
    let join_channel_button = create_button(label);
    join_channel_button.add_css_class(ACTIVE_SELECT_BUTTON_CSS);
    join_channel_button
}

pub fn create_inactive_button(label: &str) -> Button {
    let join_channel_button = create_button(label);
    join_channel_button.add_css_class(INACTIVE_SELECT_BUTTON_CSS);
    join_channel_button
}

pub fn create_combobox() -> ComboBoxText {
    ComboBoxText::builder().width_request(172).build()
}
