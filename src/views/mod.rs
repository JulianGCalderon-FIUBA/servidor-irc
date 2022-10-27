mod sidebar;

use gtk::Align;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Label;
use gtk::Orientation;
use gtk::Box;
use gtk::prelude::*;
use gtk::Application;

use sidebar::Sidebar;

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
    .default_width(700)
    .build();

    // let main_box = Box::builder()
    // .orientation(Orientation::Horizontal)
    // .build();

    // let box1 = create_box("Box1");
    // let box2 = create_box("Box2");
    // let box3 = create_box("Box3");

    // main_box.add(&box1);
    // main_box.add(&box2);
    // main_box.add(&box3);

    let sidebar = Sidebar::new();

    window.set_child(Some(&sidebar));
    
    window.show_all();
}
    
fn say_hi() {
    println!("Hi");
}

fn create_button(label: &str) -> Button {
    let button = Button::builder()
    .label(label)
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .halign(Align::Center)
    .valign(Align::Center)
    .build();

    button.connect_clicked(|_| say_hi());

    button
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

fn create_box(label: &str) -> Box {
    let gtk_box = Box::builder()
    .orientation(Orientation::Vertical)
    .build();

    let button = create_button(label);

    let label = create_label(label);

    gtk_box.add(&button);
    gtk_box.add(&label);

    gtk_box
}

