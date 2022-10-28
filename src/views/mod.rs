mod sidebar;
mod chat;
mod conv_info;

use gtk4 as gtk;

use gtk::ApplicationWindow;
use gtk::Orientation;
use gtk::Box;
use gtk::prelude::*;
use gtk::Application;

use sidebar::Sidebar;
use chat::Chat;
use conv_info::ConvInfo;

pub fn run() {
    let app = Application::new(Some("com.lemon-pie.demo"), Default::default());
    
    app.connect_activate(build_ui);
    app.run();
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
    .build();

    let sidebar = Sidebar::new();
    main_box.append(&sidebar);

    let chat = Chat::new();
    main_box.append(&chat);

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

