use gtk::{prelude::*, Align, Box, Button, Label, Orientation};
use gtk4 as gtk;

use crate::views::view_main::widgets_creation::create_button;

pub fn create_main_box_add_view() -> Box {
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .width_request(400)
        .halign(gtk::Align::Center)
        .build();
    main_box.add_css_class("main_box");
    main_box
}

pub fn create_label(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Center)
        .valign(Align::Center)
        .build()
}

pub fn create_add_channel_buton(label: &str) -> Button {
    let button = create_button(label);
    button.set_halign(gtk::Align::Center);
    button.set_valign(gtk::Align::Center);
    button
}

pub fn create_label_box(label: &str) -> Box {
    let label_input_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .margin_top(20)
        .margin_bottom(20)
        .build();
    let label = create_label(label);
    label_input_box.append(&label);
    label_input_box
}

pub fn create_title(title_text: &str) -> Label {
    let title = create_label(title_text);
    title.add_css_class("add_view_title");
    title
}
