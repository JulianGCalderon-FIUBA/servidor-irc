use gtk4 as gtk;

use gtk::{Align::Start, Label, traits::WidgetExt};

const RECEIVED_MESSAGE_CSS: &str = "received_message";
const SEND_MESSAGE_CSS: &str = "send_message";


/// Creates a gtk message label.
///
/// Receives message, returns a Label.
pub fn create_message(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(12)
        .margin_end(12)
        .build()
}

/// Creates a sent message.
///
/// Receives message, returns a Label.
pub fn create_send_message(label: &str) -> Label {
    let message = create_message(label);
    message.set_halign(gtk4::Align::End);
    message.add_css_class(SEND_MESSAGE_CSS);
    message
}

/// Creates a received message.
///
/// Receives message, returns a Label.
pub fn create_received_message(label: &str) -> Label {
    let message = create_message(label);
    message.set_halign(Start);
    message.add_css_class(RECEIVED_MESSAGE_CSS);
    message
}
