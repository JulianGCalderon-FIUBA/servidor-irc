use std::io;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use internet_relay_chat::client::Client;
use internet_relay_chat::ADDRESS;

fn main() {
    let mut client = match Client::new(ADDRESS.to_string()) {
        Ok(stream) => stream,
        Err(error) => return eprintln!("Error connecting to server: {:?}", error),
    };

    client.async_print();

    let (stdin, handle) = spawn_stdin_channel();

    loop {
        if client.finished_asnyc_read() {
            println!("Connection with server was closed, press enter to continue.");
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

    drop(stdin);
    handle.join().ok();
}

fn spawn_stdin_channel() -> (Receiver<String>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || loop {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_err() {
            return;
        }

        buffer.pop();
        if tx.send(buffer).is_err() {
            return;
        }
    });

    (rx, handle)
}
