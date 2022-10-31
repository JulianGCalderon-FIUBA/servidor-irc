use std::{cell::Cell, sync::mpsc::Sender};

use gtk4 as gtk;
use gtk::{
    Align,
    Label,
    Button,
    Entry,
    Application,
    ApplicationWindow,
    Box,
    Orientation,
    prelude::*
};

use crate::client::Client;


pub struct RegisterView {
    pub nick_entry: Entry,
    pub username_entry: Entry,
    pub pass_entry: Entry,
    pub login_button: Button,
    sender: Sender<String>
}

impl RegisterView {
    pub fn new(sender: Sender<String>) -> Self {
        Self { 
            nick_entry: create_entry(), 
            username_entry: create_entry(), 
            pass_entry: create_entry(),
            login_button: create_login_button("login"),
            sender
        }
    }

    pub fn get_view(&mut self, app: &Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder().application(app).title("Lemon Pie IRC").build();

        let main_box = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(gtk::Align::Center)
            .build();
        main_box.add_css_class("main_box");

        let nickname_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .build();
        let label = create_label("Nickname");
        nickname_box.append(&label);
        self.nick_entry = create_entry();
        nickname_box.append(&self.nick_entry);
        nickname_box.set_margin_top(20);
        nickname_box.set_margin_bottom(20);
        main_box.append(&nickname_box);

        let username_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .build();
        let label = create_label("Username");
        username_box.append(&label);
        self.username_entry = create_entry();
        username_box.append(&self.username_entry);
        username_box.set_margin_top(20);
        username_box.set_margin_bottom(20);
        main_box.append(&username_box);

        let password_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .build();
        let label = create_label("Password");
        password_box.append(&label);
        self.pass_entry = create_entry();
        password_box.append(&self.pass_entry);
        password_box.set_margin_top(20);
        password_box.set_margin_bottom(20);
        main_box.append(&password_box);

        main_box.append(&self.login_button);
        main_box.set_margin_top(20);
        main_box.set_margin_bottom(20);
        main_box.set_margin_start(20);
        main_box.set_margin_end(20);
        
        

        self.connect_button(self.pass_entry.clone(), 
                            self.nick_entry.clone(), 
                        self.username_entry.clone(),
                        self.sender.clone());

        window.set_child(Some(&main_box));

        window
    }

    fn connect_button(&self, _pass_entry: Entry, _nick_entry: Entry, _username_entry: Entry, sender: Sender<String>) {
        // let client = self.client.get_mut();
        self.login_button.connect_clicked(move |_| {
            sender.send("Hola".to_string()).expect("Error message");
        });
    }

    pub fn get_pass(&self) -> Entry {
        self.pass_entry.clone()
    }
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

fn create_login_button(label: &str) -> Button {
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

