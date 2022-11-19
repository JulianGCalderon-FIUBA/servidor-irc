use gtk4 as gtk;
use gtk::{Button, Box, Orientation, prelude::* };


pub fn create_box(orientation: Orientation) -> Box{
    Box::builder()
        .orientation(orientation)
        .margin_top(20)
        .margin_bottom(20)
        .halign(gtk::Align::Center)
        .build()
}

pub fn create_active_button(label: &str)-> Button{
    let join_channel_button = Button::builder().label(label).build();
    join_channel_button.add_css_class("active_select_button");
    join_channel_button
}

pub fn create_disable_button(label: &str)-> Button{
    let join_channel_button = Button::builder().label(label).build();
    join_channel_button.add_css_class("inactive_select_button");
    join_channel_button
}