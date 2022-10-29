use gtk4 as gtk;

use gtk::{Align, Box, Button, Entry, glib, glib::once_cell::sync::Lazy, glib::ParamSpec, Label, Orientation, prelude::*};
use gtk::subclass::prelude::{BoxImpl, ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::subclass::widget::WidgetImpl;


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

        let message_1 = create_message("sol: Hola chicos!");
        message_1.add_css_class("message");
        obj.append(&message_1);

        let message_2 = create_message("juli: Como estan?");
        message_2.add_css_class("message");
        obj.append(&message_2);

        let message_3 = create_empty_message();
        obj.append(&message_3);

        let message_4 = create_empty_message();
        obj.append(&message_4);

        let message_5 = create_empty_message();
        obj.append(&message_5);

        let message_6 = create_empty_message();
        obj.append(&message_6);

        let message_7 = create_empty_message();
        obj.append(&message_7);

        let message_8 = create_empty_message();
        obj.append(&message_8);

        let message_sender_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .halign(gtk::Align::Center)
        .build();

        let info_button = create_button("Info Personal");
        info_button.connect_clicked(|_| println!("Hi"));
        message_sender_box.append(&info_button);

        let input = create_entry("Message...");
        message_sender_box.append(&input);

        let send_button = create_button("Send");
        let text = input.text().clone();
        let _obj_clone = obj.clone();
        send_button.connect_clicked(move |_| {
            println!("{}", text);
            message_7.set_text("hola");
            message_7.add_css_class("message");
            // _obj_clone.append(&message);
        });
        message_sender_box.append(&send_button);

        obj.append(&message_sender_box);

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

fn create_message(label: &str) -> Label {
    Label::builder()
    .label(label)
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .halign(Align::Start)
    .build()
}

fn create_empty_message() -> Label {
    let message = create_message("");
    message.add_css_class("empty_message");
    message
}

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