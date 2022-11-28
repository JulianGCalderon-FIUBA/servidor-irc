use gtk::{
    traits::{WidgetExt, BoxExt},
    Align::{Center, Start},
    Box, Button, Label,
    Orientation::Vertical,
    ScrolledWindow,
};
use gtk4 as gtk;

use crate::views::widgets_creation::create_button_with_margin;

use super::{ADD_BUTTON_CSS, CURRENT_CHAT_TITLE_CSS, WELCOME_MESSAGE, WELCOME_MESSAGE_CSS, WELCOME_TITLE_CSS, WELCOME_TITLE};

pub fn create_add_button(label: &str) -> Button {
    let add_button = create_button_with_margin(label);
    add_button.add_css_class(ADD_BUTTON_CSS);
    add_button
}

pub fn create_message_box() -> Box {
    Box::builder()
        .orientation(Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_bottom(10)
        .width_request(620)
        .halign(Start)
        .build()
}

pub fn create_channels_and_client_box() -> Box {
    Box::builder()
        .orientation(Vertical)
        .height_request(200)
        .build()
}

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

pub fn create_scrollwindow_sidebar(/*child: &Box*/) -> ScrolledWindow {
    ScrolledWindow::builder().min_content_height(280).build()
}

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
