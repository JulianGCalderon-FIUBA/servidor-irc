use gtk::Button;
use gtk::glib;
use gtk::glib::ParamSpec;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::BoxImpl;
use gtk::subclass::prelude::ContainerImpl;
use gtk::subclass::prelude::ObjectImpl;
use gtk::subclass::prelude::ObjectImplExt;
use gtk::subclass::prelude::ObjectSubclass;
use gtk::subclass::widget::WidgetImpl;

#[derive(Default)]
pub struct Sidebar {
    channels: Vec<String>,
    button1: gtk::Button,
    button2: gtk::Button,
    button3: gtk::Button,
}

#[glib::object_subclass]
impl ObjectSubclass for Sidebar {
    const NAME: &'static str = "Sidebar";
    type Type = super::Sidebar;
    type ParentType = gtk::Box;
}

impl ObjectImpl for Sidebar {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed();

        // let button1 = create_button("Boton1");
        // let button2 = create_button("Boton2");
        // let button3 = create_button("Boton3");

        // self.add(&self.button1);
        // self.add(&self.button2);
        // self.add(&self.button3);
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
        Lazy::new(|| {
            vec![]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, _value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![]
        });
        SIGNALS.as_ref()
    }

    fn dispose(&self) {}
}

impl WidgetImpl for Sidebar {}

impl BoxImpl for Sidebar {}

impl ContainerImpl for Sidebar {}

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