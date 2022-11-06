use gtk::{prelude::*, Align, Box, Button, Entry, Label, ScrolledWindow};
use gtk4 as gtk;

use crate::views::view_main::widgets_creation::create_button;

pub fn create_send_button(
    message_box: Box,
    input: Entry,
    scrolled_window: ScrolledWindow,
) -> Button {
    let send_button = create_button("send");
    // send_button.add_css_class("send_button");

    send_button.connect_clicked(move |_| {
        let input_text = input.text();
        if !entry_is_valid(&input_text) {
            return;
        }

        let message = create_message(&input_text);
        message_box.append(&message);

        let adj = scrolled_window.vadjustment();
        adj.set_upper(adj.upper() + adj.page_size());
        adj.set_value(adj.upper());
        scrolled_window.set_vadjustment(Some(&adj));
    });

    send_button
}

fn entry_is_valid(entry_text: &str) -> bool {
    !entry_text.is_empty()
}

pub fn create_message(label: &str) -> Label {
    let message = Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();
    message.add_css_class("message");
    message
}
