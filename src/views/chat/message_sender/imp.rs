use gtk::Entry;
use gtk4 as gtk;

use gtk::Button;
use gtk::Label;
use gtk::Align;
use gtk::glib;
use gtk::glib::ParamSpec;
use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::subclass::prelude::BoxImpl;
use gtk::subclass::prelude::ObjectImpl;
use gtk::subclass::prelude::ObjectImplExt;
use gtk::subclass::prelude::ObjectSubclass;
use gtk::subclass::widget::WidgetImpl;

#[derive(Default)]
pub struct MessageSender {
    // buttons: Vec<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for MessageSender {
    const NAME: &'static str = "MessageSender";
    type Type = super::MessageSender;
    type ParentType = gtk::Box;
}

impl ObjectImpl for MessageSender {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let info_button = create_button("Info Personal");
        info_button.connect_clicked(|_| println!("Hi"));
        obj.append(&info_button);

        let input = create_entry("Message...");
        obj.append(&input);

        let send_button = create_button("Send");
        let text = input.text().clone();
        let obj_clone = obj.clone();
        send_button.connect_clicked(move |_| {
            println!("{}", text);
            // let message = create_message(&text);
            // obj_clone.append(&message);
        });
        obj.append(&send_button);

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

impl WidgetImpl for MessageSender {}

impl BoxImpl for MessageSender {}

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

    button
}

fn create_entry(placeholder: &str) -> Entry {
    Entry::builder().placeholder_text(placeholder).build()
}

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