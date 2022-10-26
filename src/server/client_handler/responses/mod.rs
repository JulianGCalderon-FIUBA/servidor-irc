mod errors;
mod replys;

use std::io::{self, Read, Write};

use crate::{message::Message, server::ClientHandler};

impl<T: Read + Write> ClientHandler<T> {
    pub fn send_response(&mut self, response: &str) -> io::Result<()> {
        let response = Message::new(response).unwrap();
        response.send_to(&mut self.stream_client_handler)
    }

    pub fn nickname_in_use_response(&mut self) -> io::Result<()> {
        let response = "433 :nickname is already in use".to_string();
        self.send_response(&response)
    }

    pub fn nickname_collision_response(&mut self) -> io::Result<()> {
        let response = "436 :nickname collision KILL".to_string();
        self.send_response(&response)
    }

    pub fn already_registered_response(&mut self) -> io::Result<()> {
        let response = "462 :may not reregister".to_string();
        self.send_response(&response)
    }
}
