use gtk::{Orientation, Separator};
use gtk4 as gtk;

pub fn create_separator_sidebar() -> Separator {
    Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .hexpand(true)
        .build()
}
