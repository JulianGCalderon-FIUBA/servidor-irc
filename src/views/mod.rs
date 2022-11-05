mod chat;
mod conv_info;
mod login;
mod sidebar;

use gtk::{
    gdk::Display, prelude::*, Application, ApplicationWindow, Box, Button, CssProvider,
    Orientation, Separator, StyleContext,
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
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_login(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Lemon Pie IRC")
        .build();

    let main_box = create_main_box(Orientation::Vertical, 300, 300);
    main_box.add_css_class("main_box");

    let login_content = LogIn::new();
    main_box.append(&login_content);

    let button = create_button("login", window.clone(), app.clone());
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
        window.close();
        build_main_app(&app);
    });

    button
}

fn build_main_app(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Lemon Pie IRC")
        .build();

    let main_box = create_main_box(Orientation::Horizontal, 800, 1200);
    main_box.add_css_class("main_box");

    let sidebar = Sidebar::new();
    sidebar.add_css_class("sidebar");
    main_box.append(&sidebar);

    let separator = create_separator();
    main_box.add_css_class("separator");
    main_box.append(&separator);

    let chat = Chat::new();
    chat.add_css_class("chat");
    main_box.append(&chat);

    // let separator = create_separator();
    // main_box.append(&separator);

    let conv_info = ConvInfo::new();
    main_box.append(&conv_info);

    window.set_child(Some(&main_box));
    window.fullscreen();
    window.show();
}

fn create_separator() -> Separator {
    Separator::builder()
        .orientation(Orientation::Vertical)
        .build()
}

fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(gtk::Align::Center)
        .height_request(height)
        .width_request(width)
        .build()
}
