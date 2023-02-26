use gtk::{
    traits::WidgetExt,
    Align::Start,
    Label,
};
use gtk4 as gtk;

use super::MESSAGE_SENDER_NAME_CSS;

/// Creates a sender label.
///
/// Receives nickname, returns a Label.
pub fn create_sender_nickname_label(label: &str) -> Label {
    let sender_nickname_label = Label::builder()
        .label(label)
        .margin_top(12)
        .margin_start(12)
        .halign(Start)
        .build();
    sender_nickname_label.add_css_class(MESSAGE_SENDER_NAME_CSS);
    sender_nickname_label
}
