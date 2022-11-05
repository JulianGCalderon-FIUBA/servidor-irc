use gtk::Align;
use gtk::Label;
use gtk4 as gtk;

use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::ParamSpec;
use gtk::prelude::*;
use gtk::subclass::prelude::BoxImpl;
use gtk::subclass::prelude::ObjectImpl;
use gtk::subclass::prelude::ObjectImplExt;
use gtk::subclass::prelude::ObjectSubclass;
use gtk::subclass::widget::WidgetImpl;
use gtk::Button;

#[derive(Default)]
pub struct ConvInfo {
    // buttons: Vec<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for ConvInfo {
    const NAME: &'static str = "ConvInfo";
    type Type = super::ConvInfo;
    type ParentType = gtk::Box;
}

impl ObjectImpl for ConvInfo {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let leave_button = create_button("x");
        leave_button.add_css_class("exit_channel");
        obj.append(&leave_button);

        let channel_info = create_button("Channel Info");
        obj.append(&channel_info);

        let functions_button = create_button("Functions");
        obj.append(&functions_button);

        obj.set_margin_top(12);
        obj.set_margin_bottom(12);
        obj.set_margin_start(12);
        obj.set_margin_end(12);
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

impl WidgetImpl for ConvInfo {}

impl BoxImpl for ConvInfo {}

fn create_button(label: &str) -> Button {
    let button = Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Center)
        .build();

    button.connect_clicked(|_| println!("Hi"));

    button
}

fn _create_label(label: &str) -> Label {
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
