use gtk4 as gtk;

use gtk::{ traits::WidgetExt, Align::Center, Label };

const DCC_CHATS_DESCRIPTION: &str =
    "𝘿𝙞𝙧𝙚𝙘𝙩 𝘾𝙡𝙞𝙚𝙣𝙩-𝙏𝙤-𝘾𝙡𝙞𝙚𝙣𝙩 𝙘𝙝𝙖𝙩𝙨:
• Use end-to-end encryption.
• Have no message length limit.
• Avoid delays and saturation.
• Allow a safer conversation.";

pub fn create_initial_message(nickname: &str, client: &str) -> Label {
    let label_text = format!(
        "This is a dcc chat between {} and {}

{}",
        nickname,
        client,
        DCC_CHATS_DESCRIPTION
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