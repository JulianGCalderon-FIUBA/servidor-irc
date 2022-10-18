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

    let reader = BufReader::new(stdin());
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return eprint!("Error reading from stdin: {}", error),
        };

        let message = match Message::new(&line) {
            Ok(message) => message,
            Err(error) => {
                eprintln!("Error creating message: {}", error);
                continue;
            }
        };

        if let Err(error) = client.send_message(message) {
            return eprintln!("Error sending message to server: {}", error);
        }

        match client.read_message() {
            Ok(response) => println!("{}", response),
            Err(error) => return eprintln!("Error reading response from server: {}", error),
        }
    }
}
