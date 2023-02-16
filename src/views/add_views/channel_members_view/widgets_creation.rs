use gtk4 as gtk;

use gtk::{Align::Start, Box, Button, Label, Orientation::Horizontal};

const KICK_LABEL: &str = "Kick";

/// Creates gtk box.
///
/// Receives nothing, returns a Box.
pub fn create_kick_label_box() -> Box {
    Box::builder()
        .orientation(Horizontal)
        .halign(Start)
        .margin_top(20)
        .margin_bottom(20)
        .build()
}

/// Creates gtk kick button.
///
/// Receives nothing, returns a Button.
pub fn create_kick_button() -> Button {
    Button::builder().label(KICK_LABEL).build()
}

/// Creates gtk kick label.
///
/// Receives member name, returns a Label.
pub fn create_kick_label(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Start)
        .valign(Start)
        .build()
}
