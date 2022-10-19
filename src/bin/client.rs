use std::io::stdin;
use std::io::{BufRead, BufReader};

use internet_relay_chat::client::Client;
use internet_relay_chat::message::Message;
use internet_relay_chat::ADDRESS;

fn main() {
    let mut client = match Client::new(ADDRESS.to_string()) {
        Ok(stream) => stream,
        Err(error) => return eprintln!("Error connecting to server: {:?}", error),
    };

    client.async_read(on_message);

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
}

fn on_message(message: Message) {
    println!("{}", message);
}
