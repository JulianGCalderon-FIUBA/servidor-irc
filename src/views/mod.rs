mod sidebar;
mod chat;
mod conv_info;

use gtk4 as gtk;
use gtk::{
    Align,
    Label,
    Button,
    Entry,
    Application,
    ApplicationWindow,
    Box,
    CssProvider,
    gdk::Display,
    Orientation,
    prelude::*,
    Separator,
    StyleContext,
};

use sidebar::Sidebar;
use chat::Chat;
use conv_info::ConvInfo;

pub fn run() {
    let app = Application::new(Some("com.lemon-pie.demo"), Default::default());

    app.connect_startup(|_| load_css());
    //app.connect_activate(build_ui1);
    app.connect_activate(build_ui2);
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.scss"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn build_ui1(app: &Application) {
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
    let entry = create_entry();
    nickname_box.append(&entry);
    nickname_box.set_margin_top(20);
    nickname_box.set_margin_bottom(20);
    main_box.append(&nickname_box);

    let username_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .build();
    let label = create_label("Username");
    username_box.append(&label);
    let entry = create_entry();
    username_box.append(&entry);
    username_box.set_margin_top(20);
    username_box.set_margin_bottom(20);
    main_box.append(&username_box);

    let password_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .build();
    let label = create_label("Password");
    password_box.append(&label);
    let entry = create_entry();
    password_box.append(&entry);
    password_box.set_margin_top(20);
    password_box.set_margin_bottom(20);
    main_box.append(&password_box);

    let clone = app.clone();
    let button = create_button("login", clone);
    main_box.append(&button);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    
    window.set_child(Some(&main_box));

    window.show();
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

fn create_button(label: &str, app: Application) -> Button {
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

fn build_ui2(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("Lemon Pie IRC").build();

    let main_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .build();
    main_box.add_css_class("main_box");

    let sidebar = Sidebar::new();
    sidebar.add_css_class("sidebar");
    main_box.append(&sidebar);

    let separator = create_separator();
    main_box.add_css_class("separator");
    main_box.append(&separator);

    let chat = Chat::new();
    main_box.append(&chat);

    let separator = create_separator();
    main_box.append(&separator);

    let conv_info = ConvInfo::new();
    main_box.append(&conv_info);

    main_box.set_height_request(1015);
    window.set_child(Some(&main_box));

    window.show();
}

fn create_separator() -> Separator {
    Separator::builder().orientation(Orientation::Vertical).build()
}