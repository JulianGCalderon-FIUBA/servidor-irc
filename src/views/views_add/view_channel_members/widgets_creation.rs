use gtk4 as gtk;

use gtk::{
    Box, Orientation::Horizontal, Button, Label, Align::Start,
};

const KICK_LABEL: &str = "Kick";

pub fn create_kick_label_box() -> Box {
    Box::builder()
        .orientation(Horizontal)
        .halign(Start)
        .margin_top(20)
        .margin_bottom(20)
        .build()
}

pub fn create_kick_button() -> Button {
    Button::builder().label(KICK_LABEL).build()
}

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

