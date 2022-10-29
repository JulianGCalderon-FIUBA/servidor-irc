mod sidebar;
mod chat;
mod conv_info;

use gtk4 as gtk;

use gtk::ApplicationWindow;
use gtk::StyleContext;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::Box;
use gtk::Separator;
use gtk::prelude::*;
use gtk::Application;

use gtk::gdk::Display;

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
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
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
    .margin_start(20)
    .build();

    let main_box = Box::builder()
    .orientation(Orientation::Horizontal)
    .halign(gtk::Align::Center)
    .build();

    let sidebar = Sidebar::new();
    main_box.append(&sidebar);

    let separator = create_separator();
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
    
// fn _say_hi() {
//     println!("Hi");
// }

// fn _create_button(label: &str) -> Button {
//     let button = Button::builder()
//     .label(label)
//     .margin_top(12)
//     .margin_bottom(12)
//     .margin_start(12)
//     .margin_end(12)
//     .halign(Align::Center)
//     .valign(Align::Center)
//     .build();

//     button.connect_clicked(|_| _say_hi());

//     button
// }

// fn _create_label(label: &str) -> Label {
//     Label::builder()
//     .label(label)
//     .margin_top(12)
//     .margin_bottom(12)
//     .margin_start(12)
//     .margin_end(12)
//     .halign(Align::Center)
//     .valign(Align::Center)
//     .build()
// }

// fn _create_box(label: &str) -> Box {
//     let gtk_box = Box::builder()
//     .orientation(Orientation::Vertical)
//     .build();

//     let button = _create_button(label);

//     let label = _create_label(label);

//     gtk_box.append(&button);
//     gtk_box.append(&label);

//     gtk_box
// }

fn create_separator() -> Separator{
    Separator::builder()
    .orientation(Orientation::Vertical)
    .build()
}