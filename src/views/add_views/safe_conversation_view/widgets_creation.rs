use gtk4::{
    traits::WidgetExt,
    Align::{Center, Start},
    Box, Label,
    Orientation::Vertical,
};

const DCC_CHATS_DESCRIPTION: &str = "𝘿𝙞𝙧𝙚𝙘𝙩 𝘾𝙡𝙞𝙚𝙣𝙩-𝙏𝙤-𝘾𝙡𝙞𝙚𝙣𝙩 𝙘𝙝𝙖𝙩𝙨:
• Use end-to-end encryption.
• Have no message length limit.
• Avoid delays and saturation.
• Allow a safer conversation.";

/// Creates the initial message.
///
/// Receives the sender and the receiver, returns a label
pub fn create_initial_message(nickname: &str, client: &str) -> Label {
    let label_text = format!(
        "This is a dcc chat between {} and {}

{}",
        nickname, client, DCC_CHATS_DESCRIPTION
    );
    let message = Label::builder()
        .label(&label_text)
        .margin_top(5)
        .margin_bottom(20)
        .halign(Center)
        .hexpand(false)
        .build();
    message.add_css_class("send_message");
    message
}

/// Creates gtk box.
///
/// Receives nothing, returns a Box.
pub fn create_safe_message_box() -> Box {
    Box::builder()
        .orientation(Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_bottom(10)
        .width_request(638)
        .halign(Start)
        .build()
}
