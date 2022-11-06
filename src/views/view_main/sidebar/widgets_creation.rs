use gtk4 as gtk;
use gtk::{ Separator, Orientation };

pub fn create_separator_sidebar() -> Separator {
    Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .hexpand(true)
        .build()
}