mod imp;
mod messages;
mod message_sender;

use gtk4 as gtk;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Chat(ObjectSubclass<imp::Chat>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Actionable, gtk::Buildable, gtk::Orientable;
}


impl Chat {
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Vertical)])
                                .expect("Could not be created")
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}