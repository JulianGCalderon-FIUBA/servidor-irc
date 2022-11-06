use gtk::{Button, Orientation, Separator};
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
