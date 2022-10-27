mod imp;

use gtk4 as gtk;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct MessageSender(ObjectSubclass<imp::MessageSender>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}


impl MessageSender {
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Horizontal)])
                                .expect("Could not be created")
    }
}

impl Default for MessageSender {
    fn default() -> Self {
        Self::new()
    }
}