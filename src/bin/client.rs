use std::io::stdin;
use std::io::{BufRead, BufReader};

use internet_relay_chat::client::Client;
use internet_relay_chat::message::Message;
use internet_relay_chat::ADDRESS;

fn main() {
    let mut client = match Client::new(ADDRESS.to_string()) {
        Ok(stream) => stream,
        Err(error) => return eprintln!("Error: Connecting to server: {:?}", error),
    };

    let reader = BufReader::new(stdin());
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return eprint!("Error: Reading from stdin: {}", error),
        };

        let message = match Message::new(&line) {
            Ok(message) => message,
            Err(error) => {
                eprintln!("Error: Creating message: {}", error);
                continue;
            }
        };

        if let Err(error) = client.send_message(message) {
            return eprintln!("Error: Sending message to server: {}", error);
        }

        match client.read_message() {
            Ok(response) => println!("Response: {}", response),
            Err(error) => return eprintln!("Error: Reading response from server: {}", error),
        }
    }
}
