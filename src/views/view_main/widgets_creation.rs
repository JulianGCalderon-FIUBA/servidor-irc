use gtk::{
    traits::{BoxExt, WidgetExt},
    Align::{Center, Start},
    Box, Button, Label,
    Orientation::Vertical,
    ScrolledWindow,
};
use gtk4 as gtk;

use crate::views::widgets_creation::{create_button, create_button_with_margin};

use super::{
    ADD_BUTTON_CSS, CURRENT_CHAT_TITLE_CSS, NO_NOTIFICATIONS_CSS, NO_NOTIFICATIONS_TEXT,
    WELCOME_MESSAGE, WELCOME_MESSAGE_CSS, WELCOME_TITLE, WELCOME_TITLE_CSS,
};

/// Creates gtk button with a label.
///
/// Receives a &str, returns a Button.
pub fn create_add_button(label: &str) -> Button {
    let add_button = create_button_with_margin(label);
    add_button.add_css_class(ADD_BUTTON_CSS);
    add_button
}

/// Creates gtk box.
///
/// Receives nothing, returns a Box.
pub fn create_message_box() -> Box {
    Box::builder()
        .orientation(Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_bottom(10)
        .width_request(645)
        .halign(Start)
        .build()
}

/// Creates gtk box.
///
/// Receives nothing, returns a Box.
pub fn create_channels_and_client_box() -> Box {
    Box::builder()
        .orientation(Vertical)
        .height_request(200)
        .build()
}

/// Creates gtk label with a label.
///
/// Receives a &str, returns a Label.
pub fn create_current_chat(label: &str) -> Label {
    let message = Label::builder()
        .label(label)
        .margin_top(20)
        .margin_start(20)
        .margin_end(12)
        .halign(Center)
        .build();
    message.add_css_class(CURRENT_CHAT_TITLE_CSS);
    message
}

/// Creates a gtk scrolled window.
///
/// Receives nothing, returns a ScrolledWindow.
pub fn create_scrollwindow() -> ScrolledWindow {
    ScrolledWindow::builder().min_content_height(280).build()
}

/// Creates gtk welcome box with title and message.
///
/// Receives nothing, returns a Box.
pub fn create_welcome_box() -> Box {
    let title = Label::builder()
        .label(WELCOME_TITLE)
        .margin_top(20)
        .margin_start(20)
        .margin_end(12)
        .halign(Start)
        .valign(Start)
        .build();
    title.add_css_class(WELCOME_TITLE_CSS);
    let message = Label::builder()
        .label(WELCOME_MESSAGE)
        .margin_top(20)
        .margin_start(20)
        .margin_end(12)
        .halign(Start)
        .valign(Start)
        .build();
    message.add_css_class(WELCOME_MESSAGE_CSS);
    let welcome_box = Box::builder()
        .orientation(Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_bottom(10)
        .width_request(660)
        .height_request(600)
        .valign(Start)
        .valign(Start)
        .build();
    welcome_box.append(&title);
    welcome_box.append(&message);
    welcome_box
}

/// Creates gtk button.
///
/// Receives nothing, returns a Button.
pub fn create_notifications_button() -> Button {
    let notifications_button = create_button(NO_NOTIFICATIONS_TEXT);
    notifications_button.set_margin_top(5);
    notifications_button.add_css_class(NO_NOTIFICATIONS_CSS);
    notifications_button
}
