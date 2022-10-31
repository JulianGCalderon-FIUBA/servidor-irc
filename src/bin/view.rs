// use internet_relay_chat::views::run;

// fn main() {
//     run();
// }

// use internet_relay_chat::controller_register::RegisterController;
// use gtk4 as gtk;

// use gtk::{
//     Application,
//     prelude::*, 
    // CssProvider, StyleContext, gdk::Display,
// };

// fn main() {
    // let app = Application::new(Some("com.lemon-pie.demo"), Default::default());

    // // app.connect_startup(|_| load_css());
    // // let x = 0;
    // // if x == 0 {
    // //     app.connect_activate(build_ui1);
    // // } else {
    // //     app.connect_activate(build_ui2);
    // // }    
    
    // app.connect_activate(build_ui);

    // app.run();
// }

// fn build_ui(app: &Application) {
//     let mut controller = RegisterController::new();
//     controller.start(&app);
// }

// fn load_css() {
//     let provider = CssProvider::new();
//     provider.load_from_data(include_bytes!("src/views/style.css"));

//     StyleContext::add_provider_for_display(
//         &Display::default().expect("Could not connect to a display."),
//         &provider,
//         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
//     );
// }

fn main() {}
