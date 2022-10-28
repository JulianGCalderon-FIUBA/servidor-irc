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
            let button = create_button(channel);
            obj.append(&button);
        }

        let button = create_button("+");
        obj.append(&button);

        let separator = Separator::builder()
        .orientation(Orientation::Horizontal)
        .build();
        obj.append(&separator);

        button.connect_clicked(|_| println!("Hi"));
        for conv in vec!["juli", "sol", "ana"] {
            let button = create_button(conv);
            obj.append(&button);
        }

        let button = create_button("+");
        obj.append(&button);
        
        obj.set_margin_top(12);
        obj.set_margin_bottom(12);
        obj.set_margin_start(12);
        obj.set_margin_end(12);
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

fn create_button(label: &str) -> Button {
    let button = Button::builder()
    .label(label)
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .halign(gtk::Align::Center)
    .valign(gtk::Align::Center)
    .build();

    button.connect_clicked(|_| println!("Hi"));

    button
}