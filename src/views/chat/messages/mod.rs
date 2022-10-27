mod imp;

use gtk4 as gtk;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Messages(ObjectSubclass<imp::Messages>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}


impl Messages {
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Vertical)])
                                .expect("Could not be created")
    }
}

impl Default for Messages {
    fn default() -> Self {
        Self::new()
    }
}