use gtk4::traits::WidgetExt;
use gtk4::Label;

use gtk4::Align::Start;

use crate::views::widgets_creation::create_label;

/// Creates gtk user info label.
/// 
/// Receives user info, returns a Label.
pub fn create_user_info_label(text: &str) -> Label {
    let label = create_label(text);
    label.set_halign(Start);
    label
}
