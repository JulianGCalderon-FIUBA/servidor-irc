use gtk4 as gtk;
use gtk::{ Entry, Orientation, Box };

pub fn create_entry(placeholder: &str) -> Entry {
    Entry::builder().placeholder_text(placeholder).build()
}

pub fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(gtk::Align::Center)
        .height_request(height)
        .width_request(width)
        .build()
}