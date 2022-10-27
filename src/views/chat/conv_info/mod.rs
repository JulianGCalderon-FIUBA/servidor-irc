mod imp;

use gtk4 as gtk;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ConvInfo(ObjectSubclass<imp::ConvInfo>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}


impl ConvInfo {
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Horizontal)])
                                .expect("Could not be created")
    }
}

impl Default for ConvInfo {
    fn default() -> Self {
        Self::new()
    }
}