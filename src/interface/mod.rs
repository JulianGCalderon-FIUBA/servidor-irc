mod chat;
mod conv_info;
mod login;
mod sidebar;

use gtk::{
    gdk::Display,
    prelude::*,
    Application,
    ApplicationWindow,
    Box,
    Button,
    CssProvider,
    Orientation,
    Separator,
    StyleContext,
};
use gtk4 as gtk;

use chat::Chat;
use conv_info::ConvInfo;
use login::LogIn;
use sidebar::Sidebar;

pub fn run() {
    let app = Application::new(Some("com.lemon-pie.demo"), Default::default());

    app.connect_startup(|_| load_css());

    app.connect_activate(build_login);
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

fn build_login(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("Lemon Pie IRC").build();

    let main_box = create_main_box(Orientation::Vertical, 300, 300);
    main_box.add_css_class("main_box");

    let button = create_button("login");
    main_box.append(&button);

    window.set_child(Some(&main_box));

    window.show();
}

fn create_button(label: &str, window: ApplicationWindow, app: Application) -> Button {
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
        println!("hola!");
    });

    button
}