use gtk4 as gtk;

use gtk::{traits::WidgetExt, Align::Start, Box, Button, Orientation::Horizontal};

use crate::views::widgets_creation::create_button;

const KICK_LABEL: &str = "❌";
const KICK_BUTTON_CSS: &str = "kick_button";

/// Creates gtk box.
///
/// Receives nothing, returns a Box.
pub fn create_kick_label_box() -> Box {
    Box::builder().orientation(Horizontal).halign(Start).build()
}

/// Creates gtk kick button.
///
/// Receives nothing, returns a Button.
pub fn create_kick_button() -> Button {
    let button = create_button(KICK_LABEL);
    button.add_css_class(KICK_BUTTON_CSS);

    button
}
