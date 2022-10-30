use std::io::stdin;
use std::io::{BufRead, BufReader};
use std::thread;

use gtk4::Application;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use internet_relay_chat::client::Client;
use internet_relay_chat::controller_register::RegisterController;
use internet_relay_chat::message::{CreationError, Message};
use internet_relay_chat::ADDRESS;

fn main() {
    let mut client = match Client::new(ADDRESS.to_string()) {
        Ok(stream) => stream,
        Err(error) => return eprintln!("Error connecting to server: {:?}", error),
    };

    let ui = thread::spawn(|| {
        let app = Application::new(Some("com.lemon-pie.demo"), Default::default());
        app.connect_activate(build_ui);
        app.run();
    });

    client.async_read(print_message);

    let reader = BufReader::new(stdin());
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return eprint!("Error reading from stdin: {}", error),
        };

        if let Err(error) = client.send_raw(&line) {
            return eprintln!("Error sending message to server: {}", error);
        }
    }

    ui.join().unwrap();
}

fn print_message(message: Result<Message, CreationError>) {
    match message {
        Ok(message) => println!("{}", message),
        Err(error) => eprintln!("Failed to read message: {}", error),
    };
}

fn build_ui(app: &Application) {
    let mut controller = RegisterController::new();
    controller.start(&app);
}
