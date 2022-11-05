use std::{ sync::mpsc::{ Sender, Receiver }, thread };

use gtk4 as gtk;

use crate::{
    view_register::RegisterView,
    client::Client,
    ADDRESS,
    message::{ Message, CreationError },
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
    StyleContext,
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

    pub fn start(&mut self, sender: Sender<String>, receiver: Receiver<String>) {
        let mut client = match Client::new(ADDRESS.to_string()) {
            Ok(stream) => stream,
            Err(error) => panic!("Error connecting to server: {:?}", error),
        };

        thread::spawn(move || {
            client.async_read(Self::print_message);

            for command in receiver {
                println!("{}", &command);
                client.send_raw(&command).expect("Failed sending message");
            }
        });
        self.app.connect_startup(|_| Self::load_css());
        self.app.connect_activate(move |app| Self::build_ui(&app, sender.clone()));
        self.app.run();
    }

    fn build_ui(app: &Application, sender: Sender<String>) {
        let mut view = RegisterView::new(sender);
        view.get_view(app).show();
    }

    fn print_message(message: Result<Message, CreationError>) {
        match message {
            Ok(message) => println!("{}", message),
            Err(error) => eprintln!("Failed to read message: {}", error),
        }
    }
}