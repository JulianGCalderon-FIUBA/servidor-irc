use gtk4 as gtk;

use gtk::{
    Align::Center, Box, Orientation::Horizontal,
};

pub fn create_kick_label(member: String) -> Box {
    let label_box = Box::builder()
        .orientation(Horizontal)
        .halign(Center)
        .margin_top(20)
        .margin_bottom(20)
        .build();

    label_box
}