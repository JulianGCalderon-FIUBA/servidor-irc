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

pub struct RegisterView {
    nick_entry: Entry,
    username_entry: Entry,
    pass_entry: Entry
}

impl RegisterView {
    pub fn new() -> Self {
        Self { 
            nick_entry: create_entry(), 
            username_entry: create_entry(), 
            pass_entry: create_entry() 
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

        let button = create_button("login");
        main_box.append(&button);
        main_box.set_margin_top(20);
        main_box.set_margin_bottom(20);
        main_box.set_margin_start(20);
        main_box.set_margin_end(20);
        
        window.set_child(Some(&main_box));

        window
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

    button.connect_clicked(move |_| {
        // app.connect_activate(build_ui2);
        // app.run();
    });
    
    button
}

