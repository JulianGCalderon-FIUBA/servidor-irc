use gtk4 as gtk;
use gtk::{ Align, Label, Button, prelude::* };

use crate::views::view_main::widgets_creation::create_button;

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

pub fn create_login_button(label: &str) -> Button {
    let button = create_button(label);
    button.set_halign(gtk::Align::Center);
    button.set_valign(gtk::Align::Center);
    button
}