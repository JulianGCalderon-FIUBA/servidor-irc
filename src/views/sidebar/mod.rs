mod imp;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}

// pub struct Sidebar {}

impl Sidebar {
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Vertical)])
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}