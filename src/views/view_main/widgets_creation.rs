use gtk::{Button, Orientation, Separator, Box, Label, Align, traits::WidgetExt};
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
            .halign(gtk::Align::Start)
            .build()
}

pub fn create_label(label: &str) -> Label {
    let message = Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();
    message.add_css_class("message");
    message
}
