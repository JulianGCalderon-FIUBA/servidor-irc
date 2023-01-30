use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{env, io};

use internet_relay_chat::client::Client;
use internet_relay_chat::ctcp::dcc_message::dcc_type::DccType;
use internet_relay_chat::ctcp::dcc_message::DccMessage;
use internet_relay_chat::ctcp::{get_ctcp_message, is_ctcp_message};
use internet_relay_chat::message::{CreationError, Message};
use internet_relay_chat::ADDRESS;

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = unpack_args(args);

    let mut client = match Client::new(address) {
        Ok(stream) => stream,
        Err(error) => return eprintln!("Error connecting to server: {error:?}"),
    };

    client.start_async_read(xxx);

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
            eprintln!("Error sending message to server: {error}");
            break;
        }
    }

    drop(stdin);
    handle.join().ok();
}

fn unpack_args(mut args: Vec<String>) -> String {
    args.remove(0);

    match args.pop() {
        Some(address) => address,
        None => ADDRESS.to_string(),
    }
}

/// Spawns a thread that reads from stdin, sending each line read through a channel.
/// This allows async read from stdin
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

fn _print_message(message: Result<Message, CreationError>) {
    match message {
        Ok(message) => println!("{message}"),
        Err(error) => eprintln!("{error:?}"),
    }
}

fn xxx(message: Result<Message, CreationError>) {
    let message = match message {
        Ok(message) => message,
        Err(error) => return eprintln!("{error:?}"),
    };

    if !is_ctcp_message(&message) {
        return println!("{message}");
    }

    let ctcp_message = get_ctcp_message(&message).unwrap();

    let dcc_message = match DccMessage::parse(ctcp_message) {
        Ok(dcc_message) => dcc_message,
        Err(error) => return eprintln!("{error:?}"),
    };

    match dcc_message.type_ {
        DccType::Chat => chat_command(dcc_message),
        DccType::Send => unimplemented!(),
        DccType::Unknown => eprintln!("Unknown type"),
    }
}

fn chat_command(message: DccMessage) {
    let ip = message.ip;
    let port = message.port;
    let address = format!("{ip}:{port}");

    // let dcc_connection = DccConnection::connect(address);

    println!("Connecting to {address}");
}
