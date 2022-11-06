use gtk4 as gtk;

use crate::{
    views::view_register::RegisterView,
    views::view_main::MainView,
    client::Client,
    ADDRESS,
    message::{ Message, CreationError, self },
};
use gtk::{
    gdk::Display,
    prelude::*,
    Application,
    ApplicationWindow,
    Box,
    Button,
    CssProvider,
    Orientation,
    Separator,
    StyleContext, glib::{self, Receiver, Sender},
};

pub struct Controller {
    app: Application,
}

impl Controller {
    pub fn new() -> Self {
        let app = Application::new(Some("com.lemon-pie.demo"), Default::default());

        Self {
            app,
        }
    }

    fn load_css() {
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.scss"));

        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );
    }

    pub fn start(&mut self) {
        self.app.connect_startup(|_| Self::load_css());
        self.app.connect_activate(move |app| Self::build_ui(&app));
        self.app.run();
    }

    fn build_ui(app: &Application) {

        let mut client = match Client::new(ADDRESS.to_string()) {
            Ok(stream) => stream,
            Err(error) => panic!("Error connecting to server: {:?}", error),
        }; 

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        
        let mut view = RegisterView::new(sender.clone());
        let window = view.get_view(app.clone());
        window.show();

        let app_clone = app.clone();
        
        let sender_clone = sender.clone();
        client.async_read(move |message| {
            match message {
                Ok(message) => sender_clone.send(message.to_string()).unwrap(),
                Err(error) => (eprintln!("Failed to read message: {}", error)),
            }
        });

        receiver.attach(None, move |msg| {
            match &msg[..] {
                "change"   => {
                    window.close();
                    let mut main_view = MainView::new(sender.clone());
                    main_view.get_view(app_clone.clone()).show()
                },
                "register" => {
                    client.send_raw("PASS pass123").expect("ERROR");
                    client.send_raw("NICK nick").expect("ERROR");
                    client.send_raw("USER user user user :user").expect("ERROR");
                }
                msg => println!("{}", msg)
            };
            // Returning false here would close the receiver
            // and have senders fail
            glib::Continue(true)
        });



        
    }

    // fn handle_message(msg: String, view: RegisterView) {

    // }

    // fn print_message(message: Result<Message, CreationError>) {
    //     match message {
    //         Ok(message) => println!("{}", message),
    //         Err(error) => eprintln!("Failed to read message: {}", error),
    //     }
    // }
}

// pub fn send_msg(message: Result<Message, CreationError>, sender: Sender<String>) {
    
//     match message {
//                 Ok(message) => sender.send(message.to_string()).unwrap(),
//                 Err(error) => (eprintln!("Failed to read message: {}", error)),
//             }
// }