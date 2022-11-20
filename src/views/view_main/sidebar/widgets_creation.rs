use gtk::{prelude::*, Orientation::Horizontal, Separator};
use gtk4 as gtk;

use crate::views::widgets_creation::create_separator;

pub fn create_separator_sidebar() -> Separator {
    let separator = create_separator(Horizontal);
    separator.set_margin_top(20);
    separator.set_hexpand(true);
    separator
}
