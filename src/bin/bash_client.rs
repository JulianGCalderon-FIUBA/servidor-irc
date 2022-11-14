use std::io::{self};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use internet_relay_chat::client::Client;
use internet_relay_chat::ADDRESS;

fn main() {
    let mut client = match Client::new(ADDRESS.to_string()) {
        Ok(stream) => stream,
        Err(error) => return eprintln!("Error connecting to server: {:?}", error),
    };

    client.async_print();

    let stdin = spawn_stdin_channel();

    loop {
        if !client.is_connected() {
            break;
        }

        let line = match stdin.recv_timeout(Duration::from_millis(100)) {
            Ok(line) => line,
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
        };

        if let Err(error) = client.send_raw(&line) {
            eprintln!("Error sending message to server: {}", error);
            break;
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });

    rx
}
