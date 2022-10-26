use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder().application_id("com.lemon-pie.demo").build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // let button = Button::builder().label("Click Me").margin_top(12).margin_bottom(12).margin_start(12).margin_end(12).build();

    let window = ApplicationWindow::builder().title("Lemon Pie").application(app).build();

    // button.connect_clicked(move |_| say_hi());

    window.show();
}

// fn say_hi() {
//     println!("Hi");
// }