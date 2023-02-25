pub mod dcc_chat_receiver;
pub mod dcc_chat_sender;

use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
    sync::mpsc::{self, Receiver},
    thread,
};

use magic_crypt::MagicCryptTrait;
use magic_crypt::{new_magic_crypt, MagicCrypt256};

use crate::message::{read_line, CRLF};

pub struct DccChat {
    pub stream: TcpStream,
    pub read_stream: Option<TcpStream>,
    pub magic_crypt: MagicCrypt256,
}

impl DccChat {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let read_stream = Some(stream.try_clone()?);
        let magic_crypt = new_magic_crypt!("magickey", 256);

        Ok(Self {
            stream,
            read_stream,
            magic_crypt,
        })
    }

    pub fn connect(address: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Self::new(stream)
    }

    /// Sends message to connected stream.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = self.magic_crypt.encrypt_str_to_base64(message);

        self.stream.write_all(bytes.as_bytes())?;
        self.stream.write_all(CRLF)
    }

    pub fn async_read_message(&mut self) -> Receiver<String> {
        let (sender, receiver) = mpsc::channel();

        let mut stream = self.read_stream.take().unwrap();
        let magic_crypt = self.magic_crypt.clone();
        thread::spawn(move || loop {
            while let Ok(encrypted_message) = read_message(&mut stream) {
                let message = magic_crypt
                    .decrypt_base64_to_string(&encrypted_message)
                    .unwrap();

                sender.send(message).unwrap();
            }
        });

        receiver
    }
}

fn read_message(stream: &mut TcpStream) -> io::Result<String> {
    let mut content = String::new();

    read_line(stream, &mut content)?;

    if content.as_bytes().ends_with(CRLF) {
        content.pop();
        content.pop();
    } else {
        return error_no_trailing_crlf();
    }

    Ok(content)
}

fn error_no_trailing_crlf() -> Result<String, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Message should be trailed with CRLF",
    ))
}
