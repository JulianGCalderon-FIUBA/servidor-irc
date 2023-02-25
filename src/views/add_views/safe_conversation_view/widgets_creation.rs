use gtk4 as gtk;

use gtk::{
    traits::WidgetExt,
    Align::{self, Start},
    Label,
};

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

pub fn create_initial_message(nickname: &str, client: &str) -> Label {
    let label_text = format!(
        "This is a secret chat between {} and {}

𝙎𝙚𝙘𝙧𝙚𝙩 𝙘𝙝𝙖𝙩𝙨:
• Use end-to-end encryption.
• Leave no trace on our servers.
• Have a self destruct timer.
• Do not allow forwarding.",
        nickname, client
    );
    let message = Label::builder()
        .label(&label_text)
        .margin_top(5)
        .margin_bottom(20)
        .halign(Align::Center)
        .hexpand(false)
        .build();
    message.add_css_class("send_message");
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
