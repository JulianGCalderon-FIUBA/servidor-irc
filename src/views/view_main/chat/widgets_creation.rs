use gtk::{prelude::*, Align, Box, Label, Orientation, ScrolledWindow};
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

pub fn create_chat_box() -> Box {
    let chat = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::End)
        .hexpand(true)
        .build();
    chat.add_css_class("chat");
    chat
}

pub fn create_message_sender_box() -> Box {
    Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build()
}

pub fn create_scrollwindow_chat(child: &Box) -> ScrolledWindow {
    let scrolled_window = ScrolledWindow::builder()
        .min_content_height(600)
        .max_content_width(500)
        .margin_top(20)
        .margin_start(20)
        .margin_end(20)
        .margin_bottom(20)
        .child(child)
        .build();
    scrolled_window.add_css_class("message_box");
    scrolled_window
}
