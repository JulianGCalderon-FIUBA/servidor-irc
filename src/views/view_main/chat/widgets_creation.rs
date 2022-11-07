use gtk::{prelude::*, Align, Label};
use gtk4 as gtk;

pub fn create_message(label: &str) -> Label {
    let message = Label::builder()
        .label(label)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::End)
        .build();
    message.add_css_class("send_message");
    message
}

pub fn create_received_message(label: &str) -> Label {
    let message = Label::builder()
        .label(label)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();
    message.add_css_class("received_message");
    message
}
