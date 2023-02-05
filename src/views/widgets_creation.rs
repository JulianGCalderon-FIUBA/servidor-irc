use gtk::{
    prelude::*,
    Align::{Center, End},
    ApplicationWindow, Box, Button, Entry, Label, Orientation,
    Orientation::{Horizontal, Vertical},
    PasswordEntry, ScrolledWindow, Separator,
};
use gtk4 as gtk;

use super::{APP_TITLE, CHAT_CSS, MESSAGE_BOX_CSS, WARNING_TEXT_CSS};

/// Creates gtk entry with a placeholder.
///
/// Receives a &str, returns an Entry.
pub fn create_entry(placeholder: &str) -> Entry {
    Entry::builder().placeholder_text(placeholder).build()
}

pub fn create_password_entry(placeholder: &str) -> PasswordEntry {
    PasswordEntry::builder()
        .placeholder_text(placeholder)
        .build()
}

/// Creates a gtk box with orientation, height and width.  
///
/// Receives an Orientation, i32, i32, returns a Box.
pub fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(Center)
        .height_request(height)
        .width_request(width)
        .build()
}

/// Creats gtk label with a label.  
///
/// Receives &str, returns a Label.
pub fn create_label(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Center)
        .valign(Center)
        .build()
}

/// Creates a gtk box with label.
///
/// Receives a &str, returns a Box.
pub fn create_label_input_box(label: &str) -> Box {
    let label_input_box = Box::builder()
        .orientation(Horizontal)
        .halign(Center)
        .margin_top(20)
        .margin_bottom(20)
        .build();
    let label = create_label(label);
    label_input_box.append(&label);
    label_input_box
}

/// Creates gtk button with label.  
///
/// Receives a &str, returns a Button.
pub fn create_button(label: &str) -> Button {
    Button::builder().label(label).build()
}

/// Creates gtk button with margins.  
///
/// Receives a &str, returns a Button.
pub fn create_button_with_margin(label: &str) -> Button {
    let button = create_button(label);
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);
    button
}

/// Creates a centerede gtk button with label.  
///
/// Receives a &str, returns a Button.
pub fn create_center_button(label: &str) -> Button {
    let button = create_button_with_margin(label);
    button.set_halign(Center);
    button.set_valign(Center);
    button
}

/// Creates a gtk separator with an orientation.  
///
/// Receives an Orientation, returns a Separator.
pub fn create_separator(orientation: Orientation) -> Separator {
    Separator::builder().orientation(orientation).build()
}

/// Creates a gtk window.  
///
/// Receives nothing, returns an ApplicationWindow
pub fn build_application_window() -> ApplicationWindow {
    ApplicationWindow::builder().title(APP_TITLE).build()
}

///Creates an error label.  
///
/// Receives nothing, return a Label.
pub fn create_error_label() -> Label {
    let error_label = Label::builder()
        .label("")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    error_label.add_css_class(WARNING_TEXT_CSS);
    error_label
}

/// Creates the chat box.  
///
/// Receives nothing, returns a Box.
pub fn create_chat_box() -> Box {
    let chat = Box::builder()
        .orientation(Vertical)
        .halign(Center)
        .valign(End)
        .hexpand(true)
        .build();
    chat.add_css_class(CHAT_CSS);
    chat
}

/// Creates the sender box.
///
/// Receives nothing, returns a Box.
pub fn create_message_sender_box() -> Box {
    Box::builder()
        .orientation(Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build()
}

/// Creates the scrolled window in the chat.
///
/// Receives nothing, returns a ScrolledWindow.
pub fn create_scrollwindow_chat() -> ScrolledWindow {
    let scrolled_window = ScrolledWindow::builder()
        .min_content_height(720)
        .max_content_width(500)
        .margin_top(20)
        .margin_start(20)
        .margin_end(20)
        .margin_bottom(20)
        .build();
    scrolled_window.add_css_class(MESSAGE_BOX_CSS);
    scrolled_window
}
