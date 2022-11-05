mod imp;

use gtk4 as gtk;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}

impl Sidebar {
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Vertical)]).expect("Could not be created")
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}
