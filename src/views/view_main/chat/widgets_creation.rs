use gtk::{
    prelude::*,
    Align::{Center, End, Start},
    Box, Label,
    Orientation::{Horizontal, Vertical},
    ScrolledWindow,
};
use gtk4 as gtk;

use super::{CHAT_CSS, MESSAGE_BOX_CSS, RECEIVED_MESSAGE_CSS, SEND_MESSAGE_CSS};

pub fn create_message(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn create_send_message(label: &str) -> Label {
    let message = create_message(label);
    message.set_halign(End);
    message.add_css_class(SEND_MESSAGE_CSS);
    message
}

pub fn create_received_message(label: &str) -> Label {
    let message = create_message(label);
    message.set_halign(Start);
    message.add_css_class(RECEIVED_MESSAGE_CSS);
    message
}

pub fn create_sender_nickname_label(label: &str) -> Label {
    let sender_nickname_label = Label::builder()
        .label(label)
        .margin_top(12)
        .margin_start(12)
        .halign(gtk::Align::Start)
        .build();
    sender_nickname_label.add_css_class("message_sender_name");
    sender_nickname_label
}

pub fn create_chat_box() -> Box {
    let chat = Box::builder()
        .orientation(Vertical)
        .halign(Center)
        .valign(End)
        .hexpand(true)
        .build();
    chat.add_css_class(CHAT_CSS);
    chat
}

pub fn create_message_sender_box() -> Box {
    Box::builder()
        .orientation(Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build()
}

pub fn create_scrollwindow_chat() -> ScrolledWindow {
    let scrolled_window = ScrolledWindow::builder()
        .min_content_height(600)
        .max_content_width(500)
        .margin_top(20)
        .margin_start(20)
        .margin_end(20)
        .margin_bottom(20)
        .build();
    scrolled_window.add_css_class(MESSAGE_BOX_CSS);
    scrolled_window
}
