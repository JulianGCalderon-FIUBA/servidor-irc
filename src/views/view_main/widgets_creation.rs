use gtk4 as gtk;
use gtk::{ Box, Button, Orientation, prelude::*, Separator };

pub fn create_button(label: &str) -> Button {
    let button = Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|_| println!("Hi"));

    button
}

pub fn create_separator() -> Separator {
    Separator::builder().orientation(Orientation::Vertical).build()
}

pub fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(gtk::Align::Center)
        .height_request(height)
        .width_request(width)
        .build()
}