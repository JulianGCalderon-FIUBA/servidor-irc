use gtk4 as gtk;

use gtk::subclass::prelude::{BoxImpl, ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::subclass::widget::WidgetImpl;
use gtk::{
    glib, glib::once_cell::sync::Lazy, glib::ParamSpec, prelude::*, Align, Box, Button, Entry,
    Label, Orientation, ScrolledWindow,
};

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
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_bottom(10)
            .halign(gtk::Align::Start)
            .build();

        let message_sender_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        let info_button = create_button("info");
        info_button.connect_clicked(|_| println!("Hi"));
        message_sender_box.append(&info_button);

        let input = create_entry("Message...");
        input.set_hexpand(true);
        input.set_width_request(600);
        message_sender_box.append(&input);

        let scrolled_window: ScrolledWindow = ScrolledWindow::builder()
            .min_content_height(800)
            .min_content_width(600)
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .child(&message_box)
            .build();

        scrolled_window.add_css_class("message_box");
        let send_button = create_send_button(message_box, input, scrolled_window.clone());
        message_sender_box.append(&send_button);

        obj.append(&scrolled_window);
        obj.append(&message_sender_box);

        obj.set_valign(gtk::Align::End);
        obj.set_hexpand(true);
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

fn create_button(label: &str) -> Button {
    Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build()
}

fn create_entry(placeholder: &str) -> Entry {
    Entry::builder().placeholder_text(placeholder).build()
}

fn entry_is_valid(entry_text: &str) -> bool {
    !entry_text.is_empty()
}

fn create_send_button(message_box: Box, input: Entry, scrolled_window: ScrolledWindow) -> Button {
    let send_button = create_button("send");
    send_button.add_css_class("send_button");

    send_button.connect_clicked(move |_| {
        let input_text = input.text();
        if !entry_is_valid(&input_text) {
            return;
        }

        let message = create_message(&input_text);
        message.add_css_class("message");
        message_box.append(&message);

        let adj = scrolled_window.vadjustment();
        adj.set_upper(adj.upper() + adj.page_size());
        adj.set_value(adj.upper());
        scrolled_window.set_vadjustment(Some(&adj));
    });

    send_button
}
