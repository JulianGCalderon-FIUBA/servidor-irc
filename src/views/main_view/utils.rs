use gtk4::{prelude::*, ScrolledWindow};

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
    !entry_text.is_empty() && entry_text.len() < max_characters
}
