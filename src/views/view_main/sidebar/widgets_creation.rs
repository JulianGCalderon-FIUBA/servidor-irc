use gtk4 as gtk;
use gtk::{ Orientation, Separator, ScrolledWindow, Box };

pub fn create_separator_sidebar() -> Separator {
    Separator::builder().orientation(Orientation::Horizontal).margin_top(20).hexpand(true).build()
}

pub fn create_scrollwindow_sidebar(child: &Box) -> ScrolledWindow {
    ScrolledWindow::builder().min_content_height(320).child(child).build()
}