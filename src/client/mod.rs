use std::fmt::Arguments;
use std::io::{BufReader, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::thread;
use std::{io, thread::JoinHandle};

use crate::macros::some_or_return;
use crate::message::{CreationError, Message};

/// Represents a client that can connect to a Server.
pub struct Client {
    write_stream: TcpStream,
    read_stream: Option<TcpStream>,
    read_thread: Option<JoinHandle<()>>,
}

const CRLF: &[u8; 2] = b"\r\n";

impl Client {
    /// Creates new [`Client`] connected to received address.
    pub fn new(address: String) -> io::Result<Self> {
        let write_stream = TcpStream::connect(address)?;
        let read_stream = Some(write_stream.try_clone()?);

        Ok(Self {
            write_stream,
            read_stream,
            read_thread: None,
        })
    }

    /// Spawns a thread that reads message from the connected stream
    /// Calls `on_message` method on each message
    /// Thread ends on IO error
    pub fn start_async_read<F>(&mut self, on_message: F)
    where
        F: Fn(Result<Message, CreationError>) + Send + 'static,
    {
        let on_message = Box::new(on_message);

        let read_stream = match self.read_stream.take() {
            Some(read_stream) => read_stream,
            None => return,
        };

        let handle = thread::spawn(|| async_read(read_stream, on_message));
        self.read_thread = Some(handle);
    }

    pub fn sync_read(&mut self) -> Result<Message, CreationError> {
        let read_stream = self
            .read_stream
            .as_mut()
            .expect("There should be a read stream");
        Message::read_from(read_stream)
    }

    pub fn async_print(&mut self) {
        self.start_async_read(print_message);
    }

    pub fn async_zzz(&mut self) {
        self.start_async_read(zzz_message);
    }

    /// Sends message to connected stream.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.write_stream.write_all(bytes)?;
        self.write_stream.write_all(CRLF)
    }

    /// Returns true when connection with stream finalized
    pub fn finished_asnyc_read(&self) -> bool {
        if let Some(join_handle) = &self.read_thread {
            return join_handle.is_finished();
        }

        true
    }
}

fn async_read<F>(read_stream: TcpStream, on_message: Box<F>)
where
    F: Fn(Result<Message, CreationError>) + Send + 'static,
{
    let mut reader = BufReader::new(read_stream);
    loop {
        let message = Message::read_from_buffer(&mut reader);
        if let Err(CreationError::IoError(_)) = message {
            return;
        }
        on_message(message);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(handler) = self.read_thread.take() {
            if let Err(error) = handler.join() {
                eprintln!("Error: {error:?}");
            }
        }
    }
}

fn print_message(message: Result<Message, CreationError>) {
    match message {
        Ok(message) => println!("{message}"),
        Err(error) => eprintln!("{error:?}"),
    }
}

fn zzz_message(message: Result<Message, CreationError>) {
    let message = message.unwrap();

    if !is_ctcp(&message) {
        return print_message(Ok(message));
    }

    let mut content = message.unpack().3.unwrap();

    if !is_valid_dcc_chat(&content) {
        return print!("INVALIDO");
    }

    content.remove(0);
    content.pop();

    let mut arguments: Vec<&str> = content.split(' ').collect();

    let port = arguments.pop().unwrap();
    let ip = arguments.pop().unwrap();

    let socket_address = format!("{ip}:{port}");

    let mut stream = TcpStream::connect(socket_address).unwrap();

    stream.write_all(b"NICK ana").unwrap();
    stream.write_all(CRLF).unwrap();
    stream.write_all(b"USER ana ana ana :ana").unwrap();
    stream.write_all(CRLF).unwrap()
}

fn is_ctcp(message: &Message) -> bool {
    let command = message.get_command();
    let trailing: Vec<char> = message.get_trailing().as_ref().unwrap().chars().collect();

    if command != "PRIVMSG" {
        return false;
    }

    if *trailing.first().unwrap() != 1 as char {
        return false;
    }

    if *trailing.last().unwrap() != 1 as char {
        return false;
    }

    true
}

fn is_valid_dcc_chat(content: &str) -> bool {
    // let mut arguments: Vec<&str> = content.split(' ').collect();
    // let port = some_or_return!(arguments.pop(), false);
    // let address = some_or_return!(arguments.pop(), false);
    // let arg = some_or_return!(arguments.pop(), false);
    // let ty = some_or_return!(arguments.pop(), false);
    // let dcc = some_or_return!(arguments.pop(), false);

    true
}

fn ctc_start_listener() -> TcpListener {
    todo!()
}
