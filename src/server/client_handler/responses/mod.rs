pub mod errors;
mod replies;

use errors::ErrorReply;
use std::io;

use crate::{
    message::Message,
    server::{client_trait::ClientTrait, ClientHandler},
};

impl<T: ClientTrait> ClientHandler<T> {
    pub fn send_response(&mut self, response: &str) -> io::Result<()> {
        let response = Message::new(response).unwrap();
        response.send_to(&mut self.stream)
    }

    pub fn send_response_for_error(&mut self, error: ErrorReply) -> io::Result<()> {
        self.send_response(&error.to_string())
    }
}
