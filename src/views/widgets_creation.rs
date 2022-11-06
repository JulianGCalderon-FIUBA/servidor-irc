use gtk4 as gtk;
use gtk::{ Entry };

pub fn create_entry(placeholder: &str) -> Entry {
    Entry::builder().placeholder_text(placeholder).build()
}