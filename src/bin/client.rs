use std::io::stdin;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use internet_relay_chat::message::Message;
use internet_relay_chat::ADDRESS;

fn main() {
    let mut stream = match TcpStream::connect(ADDRESS) {
        Ok(stream) => stream,
        Err(error) => return eprintln!("Error: Connecting to server: {:?}", error),
    };

    let reader = BufReader::new(stdin());
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return eprint!("Error: Reading from stdin: {}", error),
        };

        let message = Message::new(line);

        if let Err(error) = message.send_to(&mut stream) {
            return eprintln!("Error: Sending message to server: {}", error);
        }

        match Message::read_from(&mut stream) {
            Ok(Some(response)) => println!("Response: {}", response),
            Err(error) => return eprintln!("Error: Reading response from server: {}", error),
            Ok(None) => return eprintln!("EOF: Conection with server closed"),
        }
    }
}
