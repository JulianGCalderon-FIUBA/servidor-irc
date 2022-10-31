mod errors;
mod replies;

use std::io;

use crate::{message::Message, server::ClientHandler};

impl ClientHandler {
    pub fn send_response(&mut self, response: &str) -> io::Result<()> {
        let response = Message::new(response).unwrap();
        response.send_to(&mut self.stream)
    }
}
