use std::{sync::mpsc::{Sender, Receiver}, thread};

use gtk4 as gtk;

use crate::{view_register::RegisterView, client::Client, ADDRESS, message::{Message, CreationError}};
use gtk::{prelude::*, Application};

pub struct Controller {
    app: Application
}

impl Controller {
    pub fn new() -> Self {

        let app = Application::new(Some("com.lemon-pie.demo"), Default::default());
    
        Self {
            app
        }
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
        };
    }
}
