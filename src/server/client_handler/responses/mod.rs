mod errors;
mod replies;

use std::io::{self, Read, Write};

use crate::{message::Message, server::ClientHandler};

impl<T: Read + Write> ClientHandler<T> {
    pub fn send_response(&mut self, response: &str) -> io::Result<()> {
        let response = Message::new(response).unwrap();
        response.send_to(&mut self.stream_client_handler)
    }
}
