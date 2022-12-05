use gtk4::prelude::WidgetExt;
use gtk4::{Align::Start, Box, Orientation::Vertical, ScrolledWindow};

use crate::views::view_main::widgets_creation::create_scrollwindow;

pub fn create_notifications_scrollwindow() -> ScrolledWindow {
    let scrollwindow = create_scrollwindow();
    scrollwindow.set_margin_start(20);
    scrollwindow.set_margin_end(20);
    scrollwindow.add_css_class("notifications_container");
    scrollwindow
}

pub fn create_box_container() -> Box {
    Box::builder().orientation(Vertical).valign(Start).build()
}
