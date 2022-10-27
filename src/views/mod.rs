use gtk::ApplicationWindow;
use gtk::Button;
use gtk::prelude::*;
use gtk::Application;


pub fn run() {
    let app = Application::new(Some("com.lemon-pie.demo"), Default::default());
    
    app.connect_activate(build_ui);
    app.run();
}
    
fn build_ui(app: &Application) {
    
    let window = ApplicationWindow::new(app);

    window.set_title("Lemon Pie IRC");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 700);
    
    let button = Button::builder().label("Click Me").margin_top(12).margin_bottom(12).margin_start(12).margin_end(12).build();
    button.connect_clicked(|_| say_hi());

    window.set_child(Some(&button));
    
    window.show_all();
}
    
fn say_hi() {
    println!("Hi");
}


