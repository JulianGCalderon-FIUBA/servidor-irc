use std::{io, ops::DerefMut};

use crate::{
    message::Message,
    server::{client_handler::ClientHandler, client_trait::ClientTrait},
};

use crate::server::client_handler::responses::errors::ErrorReply;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn message_for_command(&self, command: &str, receiver: &str, content: &str) -> Message {
        let nickname = self.registration.nickname().unwrap();
        let message = format!(":{nickname} {command} {receiver} :{content}",);

        Message::new(&message).unwrap()
    }

    pub fn send_message_to_target(&mut self, message: &Message, receiver: &str) -> io::Result<()> {
        if self.database.contains_client(receiver) {
            if let Some(error) = self.send_message_to_client(receiver, message) {
                self.send_response_for_error(error)?;
            }
        } else {
            self.send_message_to_channel(receiver, message);
        }

        Ok(())
    }

    // pub fn away_response_for_client(&mut self, nickname: &str) {}

    pub fn send_message_to_channel(&self, channel: &str, message: &Message) {
        let clients = self.database.get_clients(channel);

        for client in clients {
            self.send_message_to_client(&client, message);
        }
    }

    pub fn send_message_to_client(&self, nickname: &str, message: &Message) -> Option<ErrorReply> {
        if self.try_send_message_to_client(nickname, message).is_none() {
            let nickname = nickname.to_string();
            return Some(ErrorReply::ClientOffline { nickname });
        }

        None
    }

    fn try_send_message_to_client(&self, nickname: &str, message: &Message) -> Option<()> {
        let stream_ref = self.database.get_stream(nickname)?;
        let mut stream = stream_ref.lock().unwrap();
        message.send_to(stream.deref_mut()).ok()
    }
}
