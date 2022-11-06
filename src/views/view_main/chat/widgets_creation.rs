use gtk::{prelude::*, Align, Box, Button, Entry, Label, ScrolledWindow};
use gtk4 as gtk;

use crate::views::view_main::widgets_creation::create_button;

pub fn create_message(label: &str) -> Label {
    let message = Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();
    message.add_css_class("message");
    message
}
