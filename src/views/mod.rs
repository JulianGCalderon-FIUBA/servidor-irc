use gtk::ApplicationWindow;
use gtk::Button;
use gtk::prelude::*;
use gtk::Application;


pub fn run() {
    let app = Application::builder().application_id("com.lemon-pie.demo").build();
    
    app.connect_activate(build_ui);
    app.run();
}
    
fn build_ui(app: &Application) {
    
    
    let window = ApplicationWindow::builder().title("Lemon Pie").application(app).build();
    
    let button = Button::builder().label("Click Me").margin_top(12).margin_bottom(12).margin_start(12).margin_end(12).build();
    button.connect_clicked(move |_| say_hi());

    window.add(&button);    
    
    window.show_all();
}
    
fn say_hi() {
    println!("Hi");
}


