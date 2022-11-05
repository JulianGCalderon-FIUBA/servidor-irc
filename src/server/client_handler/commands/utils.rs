use std::io;

use crate::{
    message::Message,
    server::{client_handler::ClientHandler, client_trait::ClientTrait},
};

impl<T: ClientTrait> ClientHandler<T> {
    pub fn send_message_to_channel(&self, channel: &str, content: &str) {
        let clients = self.database.get_clients_for_channel(channel);

        for client in clients {
            self.send_message_to_client(&client, content).ok();
        }
    }

    pub fn send_message_to_client(&self, nickname: &str, content: &str) -> io::Result<()> {
        let mut stream = self.database.get_stream(nickname)?;

        let message = Message::new(content).unwrap();

        message.send_to(&mut stream)
    }

    pub fn send_message_to_target(&mut self, content: &str, receiver: &str) -> io::Result<()> {
        if self.database.contains_client(receiver) {
            self.send_message_to_client(receiver, content)?
        } else {
            self.send_message_to_channel(receiver, content);
        }

        Ok(())
    }
}
