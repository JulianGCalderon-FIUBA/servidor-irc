use gtk4::{
    traits::WidgetExt,
    Box,
    Orientation::{Horizontal, Vertical},
    Separator,
};

use crate::views::widgets_creation::create_separator;

/// Creates a gtk separator.
///
/// Receives nothing, returns a Separator.
pub fn create_separator_sidebar() -> Separator {
    let separator = create_separator(Horizontal);
    separator.set_margin_top(20);
    separator.set_hexpand(true);
    separator
}

/// Creates the sidebar box.
///
/// Returns the box.
pub fn create_sidebar_box() -> Box {
    Box::builder()
        .width_request(200)
        .orientation(Vertical)
        .build()
}
