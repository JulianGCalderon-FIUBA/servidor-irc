use gtk::{ prelude::*, Align::Center, Box, Label, Orientation::Vertical };
use gtk4 as gtk;

use crate::views::{ MAIN_BOX_CSS, widgets_creation::create_label };

use super::ADD_VIEW_TITLE_CSS;

pub fn create_main_box_add_view() -> Box {
    let main_box = Box::builder().orientation(Vertical).width_request(400).halign(Center).build();
    main_box.add_css_class(MAIN_BOX_CSS);
    main_box
}

pub fn create_title(title_text: &str) -> Label {
    let title = create_label(title_text);
    title.add_css_class(ADD_VIEW_TITLE_CSS);
    title
}