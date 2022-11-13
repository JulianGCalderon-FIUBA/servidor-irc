use gtk::{traits::WidgetExt, Align, Box, Button, Label, Orientation, Separator};
use gtk4 as gtk;

pub fn create_button(label: &str) -> Button {
    Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn create_add_button(label: &str) -> Button {
    let add_button = create_button(label);
    add_button.add_css_class("add");
    add_button
}

pub fn create_separator() -> Separator {
    Separator::builder()
        .orientation(Orientation::Vertical)
        .build()
}

pub fn create_message_box() -> Box {
    Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_bottom(10)
        .width_request(620)
        .halign(gtk::Align::Start)
        .build()
}

pub fn create_channels_box()-> Box {
    Box::builder()
        .orientation(Orientation::Vertical)
        .height_request(300)
        .height_request(200)
        .build()
}

pub fn create_clients_box()-> Box {
    Box::builder()
        .orientation(Orientation::Vertical)
        .height_request(300)
        .height_request(200)
        .build()
}

pub fn create_current_chat(label: &str) -> Label {
    let message = Label::builder()
        .label(label)
        .margin_top(20)
        .margin_start(20)
        .margin_end(12)
        .halign(Align::Center)
        .build();
    message.add_css_class("current_chat");
    message
}
