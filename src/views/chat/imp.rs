use gtk4 as gtk;

use gtk::{
    Align,
    Box,
    Button,
    Entry,
    glib,
    glib::once_cell::sync::Lazy,
    glib::ParamSpec,
    Label,
    Orientation,
    prelude::*,
};
use gtk::subclass::prelude::{ BoxImpl, ObjectImpl, ObjectImplExt, ObjectSubclass };
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

        let message_box = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Start)
            .build();

        let message_sender_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Start)
            .build();

        let info_button = create_button("info");
        info_button.connect_clicked(|_| println!("Hi"));
        info_button.set_width_request(100);
        message_sender_box.append(&info_button);

        let input = create_entry("Message...");
        input.set_hexpand(true);
        message_sender_box.append(&input);

        let send_button = create_send_button(message_box.clone(), input.clone());
        send_button.set_width_request(100);
        message_sender_box.append(&send_button);

        obj.append(&message_box);
        obj.append(&message_sender_box);

        obj.set_halign(gtk::Align::Start);
        obj.set_valign(gtk::Align::End);
        obj.set_hexpand(true);
        obj.set_width_request(1420);
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| { vec![] });
        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        _value: &glib::Value,
        pspec: &glib::ParamSpec
    ) {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
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
        .valign(Align::Start)
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
    Entry::builder()
    .placeholder_text(placeholder)
    .width_request(1150)
    .build()
}

fn entry_is_valid(entry_text: &str) -> bool {
    entry_text != ""
}

fn create_send_button(message_box: Box, input: Entry) -> Button {
    let send_button = create_button("Send!");
    
    send_button.connect_clicked(move |_| {
        let input_text = input.text();
        if !entry_is_valid(&input_text) { return }

        // if messages.iter().any(|message| message.text() == "") {
        //     let new_message = messages.iter().find(|message| message.text() == "").unwrap();
        //     new_message.add_css_class("message");
        //     new_message.set_text(&entry_text);   
        // } else {
        //     for i in 0..messages.len()-1 {
        //         messages[i].set_text(&messages[i+1].text());
        //     }
        //     messages[messages.len()-1].set_text(&entry_text);
        // }
        let message = create_message(&input_text);
        message.add_css_class("message");
        message_box.append(&message);
    });

    send_button
}