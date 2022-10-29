use gtk4 as gtk;

use gtk::{glib, glib::once_cell::sync::Lazy, glib::ParamSpec, prelude::*};
use gtk::subclass::prelude::{BoxImpl, ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::subclass::widget::WidgetImpl;

use super::messages::Messages;
use super::message_sender::MessageSender;


#[derive(Default)]
pub struct Chat {
    // buttons: Vec<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for Chat {
    const NAME: &'static str = "Chat";
    type Type = super::Chat;
    type ParentType = gtk::Box;
}

impl ObjectImpl for Chat {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let messages = Messages::new();
        obj.append(&messages);

        let message_sender = MessageSender::new();
        obj.append(&message_sender);
        
        obj.set_halign(gtk::Align::Start);
        obj.set_valign(gtk::Align::End);
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
        Lazy::new(|| {
            vec![]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self,_obj: &Self::Type,_id: usize, _value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }

    fn property(&self,_obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for Chat {}

impl BoxImpl for Chat {}
