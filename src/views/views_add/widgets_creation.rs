use gtk::{prelude::*, Align::Center, Box, Label, Orientation::Vertical};
use gtk4 as gtk;

use crate::views::{widgets_creation::create_label, MAIN_BOX_CSS};

use super::ADD_VIEW_TITLE_CSS;

/// Creates gtk main box.
/// 
/// Receives nothing, returns a Box.
pub fn create_main_box_add_view() -> Box {
    let main_box = Box::builder()
        .orientation(Vertical)
        .width_request(400)
        .halign(Center)
        .build();
    main_box.add_css_class(MAIN_BOX_CSS);
    main_box
}

/// Creates gtk title label.
/// 
/// Receives title text, returns a Label.
pub fn create_title(title_text: &str) -> Label {
    let title = create_label(title_text);
    title.add_css_class(ADD_VIEW_TITLE_CSS);
    title
}
