use gtk4::{
    traits::{AdjustmentExt, ButtonExt, WidgetExt},
    Button, ScrolledWindow,
};

use crate::controller::utils::is_not_empty;

use super::{CHAT_BUTTON_SELECTED_CSS, NOTIFICATION_ON_BUTTON_CSS};

/// Adjusts scrollbar in a scrolled window.
pub fn adjust_scrollbar(scrolled_window: ScrolledWindow) {
    let adj = scrolled_window.vadjustment();
    adj.set_upper(adj.upper() + adj.page_size());
    adj.set_value(adj.upper());
    scrolled_window.set_vadjustment(Some(&adj));
}

/// Checks if entry is valid and returns a bool.
///
/// Checks if entry is empty and if it does not exceed the amount of characters.
pub fn entry_is_valid(entry_text: &str, max_characters: usize) -> bool {
    is_not_empty(entry_text) && entry_text.len() < max_characters
}

/// Get number of notifications.
///
/// Returns u32.
pub fn get_notifications_number(button: &Button) -> u32 {
    const RADIX: u32 = 10;
    let notifications_text = button.label().unwrap().to_string();
    let number_text = *notifications_text
        .split('(')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap();
    number_text.to_digit(RADIX).unwrap()
}

pub fn button_has_notifications(button: &Button) -> bool {
    button.has_css_class(NOTIFICATION_ON_BUTTON_CSS)
}

pub fn remove_button_notifications(button: &Button, no_notifications_text: &str) {
    button.set_label(no_notifications_text);
    button.remove_css_class(NOTIFICATION_ON_BUTTON_CSS);
}

pub fn remove_button_notifications_if_any(button: &Button, no_notifications_text: &str) {
    if button_has_notifications(button) {
        remove_button_notifications(button, no_notifications_text);
    }
}

pub fn add_notification_to_button(button: &Button, button_text: String) {
    let mut notifications_number = 0;
    if button_has_notifications(button) {
        notifications_number = get_notifications_number(button);
    } else {
        button.add_css_class(NOTIFICATION_ON_BUTTON_CSS);
    }

    button.set_label(&format!("{} ({})", button_text, notifications_number + 1));
}

pub fn select_conversation_button(button: &Button) {
    button.add_css_class(CHAT_BUTTON_SELECTED_CSS);
}

pub fn deselect_conversation_button(button: &Button) {
    button.remove_css_class(CHAT_BUTTON_SELECTED_CSS);
}
