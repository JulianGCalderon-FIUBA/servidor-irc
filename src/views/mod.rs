use gtk::Align;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Label;
use gtk::Orientation;
use gtk::Box;
use gtk::prelude::*;
use gtk::Application;


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

    let button = create_button("Boton1");

    let label = create_label("Hello");

    button.connect_clicked(|_| say_hi());

    let main_box = Box::builder()
    .orientation(Orientation::Vertical)
    .build();

    main_box.add(&button);
    main_box.add(&label);

    window.set_child(Some(&main_box));
    
    window.show_all();
}
    
fn say_hi() {
    println!("Hi");
}

fn create_button(label: &str) -> Button {
    Button::builder()
    .label(label)
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .halign(Align::Center)
    .valign(Align::Center)
    .build()
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

