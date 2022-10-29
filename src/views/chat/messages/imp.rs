use gtk::Align;
use gtk::Label;
use gtk4 as gtk;

use gtk::{glib, glib::once_cell::sync::Lazy, glib::ParamSpec, prelude::*};
use gtk::subclass::prelude::{BoxImpl, ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::subclass::widget::WidgetImpl;

#[derive(Default)]
pub struct Messages {
    // buttons: Vec<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for Messages {
    const NAME: &'static str = "Messages";
    type Type = super::Messages;
    type ParentType = gtk::Box;
}

impl ObjectImpl for Messages {
    fn constructed(&self, obj: &Self::Type/*, messages: usize*/) {
        self.parent_constructed(obj);

        // for message in messages {
        //     let info_conv = create_message(message);
        //     obj.append(&info_conv);
        // }

        let info_conv = create_message("sol: Hola chicos!");
        info_conv.add_css_class("message");
        obj.append(&info_conv);

        let info_conv = create_message("juli: Como estan?");
        info_conv.add_css_class("message");
        obj.append(&info_conv);

        let info_conv = create_message("");
        info_conv.add_css_class("empty_message");
        obj.append(&info_conv);

        let info_conv = create_message("");
        info_conv.add_css_class("empty_message");
        obj.append(&info_conv);

        let info_conv = create_message("");
        info_conv.add_css_class("empty_message");
        obj.append(&info_conv);

        let info_conv = create_message("");
        info_conv.add_css_class("empty_message");
        obj.append(&info_conv);

        let info_conv = create_message("");
        info_conv.add_css_class("empty_message");
        obj.append(&info_conv);

        let info_conv = create_message("");
        info_conv.add_css_class("empty_message");
        obj.append(&info_conv);


        obj.set_margin_top(12);
        obj.set_margin_bottom(12);
        obj.set_margin_start(12);
        obj.set_margin_end(12);
        obj.set_halign(gtk::Align::Start);
    }

    // fn add_message(&self, text: str){
    //     let info_conv = create_message(text);
    //     obj.append(&info_conv);
    // }

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

impl WidgetImpl for Messages {}

impl BoxImpl for Messages {}

fn create_message(label: &str) -> Label {
    Label::builder()
    .label(label)
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .halign(Align::Center)
    .valign(Align::Center)
    .build()
}