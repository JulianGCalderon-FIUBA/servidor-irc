use gtk4 as gtk;

use gtk::Button;
use gtk::Separator;
use gtk::glib;
use gtk::glib::ParamSpec;
use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::subclass::prelude::BoxImpl;
use gtk::subclass::prelude::ObjectImpl;
use gtk::subclass::prelude::ObjectImplExt;
use gtk::subclass::prelude::ObjectSubclass;
use gtk::subclass::widget::WidgetImpl;
use gtk::Orientation;

#[derive(Default)]
pub struct Sidebar {
    // buttons: Vec<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for Sidebar {
    const NAME: &'static str = "Sidebar";
    type Type = super::Sidebar;
    type ParentType = gtk::Box;
}

impl ObjectImpl for Sidebar {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        // CONSEGUIR LISTA CANALES CON CONTROLER
        for channel in vec!["#canal1", "#canal2"] {
            let button = create_button(channel, 12);
            obj.append(&button);
        }

        let button = create_button("+", 12);
        obj.append(&button);

        let separator = create_separator();
        obj.append(&separator);

        for conv in vec!["juli", "sol", "ana"] {
            let button = create_button(conv, 12);
            obj.append(&button);
        }

        let button = create_button("+", 12);
        obj.append(&button);
        
        obj.set_margin_top(12);
        
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

impl WidgetImpl for Sidebar {}

impl BoxImpl for Sidebar {}

fn create_button(label: &str, margin: i32) -> Button {
    let button = Button::builder()
    .label(label)
    .margin_top(margin)
    .margin_bottom(margin)
    .margin_start(margin)
    .margin_end(margin)
    .valign(gtk::Align::Center)
    .build();

    button.connect_clicked(|_| println!("Hi"));

    button
}

fn create_separator() -> Separator{
    Separator::builder()
    .orientation(Orientation::Horizontal)
    .margin_top(20)
    .margin_bottom(20)
    .hexpand(true)
    .build()
}