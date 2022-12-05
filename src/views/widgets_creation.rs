use gtk::{
    prelude::*, Align::Center, ApplicationWindow, Box, Button, Entry, Label, Orientation,
    Orientation::Horizontal, Separator,
};
use gtk4 as gtk;

use super::APP_TITLE;

pub fn create_entry(placeholder: &str) -> Entry {
    Entry::builder().placeholder_text(placeholder).build()
}

pub fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(Center)
        .height_request(height)
        .width_request(width)
        .build()
}

pub fn create_label(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Center)
        .valign(Center)
        .build()
}

pub fn create_label_input_box(label: &str) -> Box {
    let label_input_box = Box::builder()
        .orientation(Horizontal)
        .halign(Center)
        .margin_top(20)
        .margin_bottom(20)
        .build();
    let label = create_label(label);
    label_input_box.append(&label);
    label_input_box
}

pub fn create_button(label: &str) -> Button {
    Button::builder().label(label).build()
}

pub fn create_button_with_margin(label: &str) -> Button {
    let button = create_button(label);
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);
    button
}

pub fn create_center_button(label: &str) -> Button {
    let button = create_button_with_margin(label);
    button.set_halign(Center);
    button.set_valign(Center);
    button
}

pub fn create_separator(orientation: Orientation) -> Separator {
    Separator::builder().orientation(orientation).build()
}

pub fn build_application_window() -> ApplicationWindow {
    ApplicationWindow::builder().title(APP_TITLE).build()
}

pub fn create_error_label() -> Label {
    let error_label = Label::builder()
        .label("")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    error_label.add_css_class("warning_text");
    error_label
}
