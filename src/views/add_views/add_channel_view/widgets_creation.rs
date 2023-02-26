use gtk4 as gtk;
use gtk::{traits::WidgetExt, Box, Button, ComboBoxText, Orientation, Align::Center};

use crate::views::widgets_creation::create_button;

use super::{ACTIVE_SELECT_BUTTON_CSS, INACTIVE_SELECT_BUTTON_CSS};

/// Creates gtk box with orientation.
///
/// Receives Orientation, returns a Box.
pub fn create_box(orientation: Orientation) -> Box {
    Box::builder()
        .orientation(orientation)
        .margin_top(20)
        .margin_bottom(20)
        .halign(Center)
        .build()
}

/// Creates gtk active button with label.
///
/// Receives &str, returns a Button.
pub fn create_active_button(label: &str) -> Button {
    let join_channel_button = create_button(label);
    join_channel_button.add_css_class(ACTIVE_SELECT_BUTTON_CSS);
    join_channel_button
}

/// Creates gtk inactive button with label.
///
/// Receives &str, returns a Button.
pub fn create_inactive_button(label: &str) -> Button {
    let join_channel_button = create_button(label);
    join_channel_button.add_css_class(INACTIVE_SELECT_BUTTON_CSS);
    join_channel_button
}

/// Creates gtk combo box.
///
/// Receives nothing, returns a ComboBoxText.
pub fn create_combobox() -> ComboBoxText {
    ComboBoxText::builder().width_request(172).build()
}
