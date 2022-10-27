use std::{io, ops::DerefMut};

use crate::{
    message::Message,
    server::{client_handler::ClientHandler, client_trait::ClientTrait},
};

use crate::server::client_handler::responses::errors::ErrorReply;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn build_text_message(&self, command: &str, receiver: &str, content: &str) -> Message {
        let message = format!(
            ":{} {} {} :{}",
            self.registration.nickname().unwrap(),
            command,
            receiver,
            content
        );

        Message::new(&message).unwrap()
    }

    pub fn send_message_to(&mut self, receiver: &str, message: &Message) -> io::Result<()> {
        if self.database.contains_client(receiver) {
            if let Some(error) = self.send_message_to_client(receiver, message) {
                self.send_response_for_error(error)?;
            }
        } else {
            self.send_message_to_channel(receiver, message);
        }

        Ok(())
    }

    pub fn send_message_to_channel(&self, channel: &str, message: &Message) {
        let clients = self.database.get_clients(channel);

        for client in clients {
            self.send_message_to_client(&client, message);
        }
    }

    pub fn send_message_to_client(&self, nickname: &str, message: &Message) -> Option<ErrorReply> {
        let stream_ref = match self.database.get_stream(nickname) {
            Some(stream_ref) => stream_ref,
            None => {
                return Some(ErrorReply::ClientOffline {
                    nickname: nickname.to_string(),
                })
            }
        };

        let mut stream = stream_ref.lock().unwrap();
        if message.send_to(stream.deref_mut()).is_err() {
            return Some(ErrorReply::ClientOffline {
                nickname: nickname.to_string(),
            });
        };

        None
    }
}
