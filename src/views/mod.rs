mod sidebar;
mod chat;
mod conv_info;

use gtk4 as gtk;
use gtk::{Application, ApplicationWindow, Box, CssProvider, gdk::Display, Orientation, prelude::*, Separator, StyleContext};

use sidebar::Sidebar;
use chat::Chat;
use conv_info::ConvInfo;

pub fn run() {
    let app = Application::new(Some("com.lemon-pie.demo"), Default::default());
    
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
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
    
fn build_ui(app: &Application) {
    
    let window = ApplicationWindow::builder()
    .application(app)
    .title("Lemon Pie IRC")
    .default_height(600)
    .default_width(1200)
    .build();
    
    let main_box = Box::builder()
    .orientation(Orientation::Horizontal)
    .margin_top(20)
    .margin_bottom(20)
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

    window.set_child(Some(&main_box));
    
    window.show();
}

fn create_separator() -> Separator{
    Separator::builder()
    .orientation(Orientation::Vertical)
    .build()
}