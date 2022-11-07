/// This module contains error responses.
pub mod errors;
/// This module contains notification responses.
pub mod notifications;
/// This module contains replies.
pub mod replies;

use errors::ErrorReply;
use std::io;

use crate::message::Message;
use crate::server::client_trait::Connection;
use crate::server::ClientHandler;

use self::replies::CommandResponse;

impl<C: Connection> ClientHandler<C> {
    /// Sends response as Message.
    pub fn send_response(&mut self, response: &str) -> io::Result<()> {
        let response = Message::new(response).unwrap();
        response.send_to(&mut self.stream)
    }

    /// Sends response for ErrorReply.
    pub fn send_response_for_error(&mut self, error: ErrorReply) -> io::Result<()> {
        self.send_response(&error.to_string())
    }

    /// Sends response for CommandResponse.
    pub fn send_response_for_reply(&mut self, reply: CommandResponse) -> io::Result<()> {
        self.send_response(&reply.to_string())
    }
}
