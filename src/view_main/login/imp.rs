use gtk4 as gtk;

use gtk::subclass::prelude::{BoxImpl, ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::subclass::widget::WidgetImpl;
use gtk::{
    glib, glib::once_cell::sync::Lazy, glib::ParamSpec, prelude::*, Align, Box, Entry, Label,
    Orientation,
};

#[derive(Default)]
pub struct LogIn {}

#[glib::object_subclass]
impl ObjectSubclass for LogIn {
    const NAME: &'static str = "LogIn";
    type Type = super::LogIn;
    type ParentType = gtk::Box;
}

impl ObjectImpl for LogIn {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let realname_box = create_label_input_line("Your name:");
        obj.append(&realname_box);

        let nickname_box = create_label_input_line("Nickname:");
        obj.append(&nickname_box);

        let username_box = create_label_input_line("Username:");
        obj.append(&username_box);

        let password_box = create_label_input_line("Password:");
        obj.append(&password_box);
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(Vec::new);
        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        _value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        pspec.name();
        unimplemented!()
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        pspec.name();
        unimplemented!()
    }
}

impl WidgetImpl for LogIn {}

impl BoxImpl for LogIn {}

fn create_label_input_line(label: &str) -> Box {
    let line_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .build();

    let label = create_label(label);
    line_box.append(&label);

    let entry = create_entry();
    line_box.append(&entry);

    line_box.set_margin_top(20);
    line_box.set_margin_bottom(20);
    line_box
}

fn create_label(label: &str) -> Label {
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

fn create_entry() -> Entry {
    Entry::builder().build()
}
