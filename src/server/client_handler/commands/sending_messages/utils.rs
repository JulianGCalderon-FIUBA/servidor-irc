use std::{
    io::{self, Read, Write},
    ops::DerefMut,
};

use crate::{message::Message, server::client_handler::ClientHandler};

impl<T: Read + Write> ClientHandler<T> {
    pub fn build_text_message(&self, command: &str, receiver: &str, content: &str) -> Message {
        let message = format!(
            ":{} {} {} :{}",
            self.connection.nickname().unwrap(),
            command,
            receiver,
            content
        );

        Message::new(&message).unwrap()
    }

    pub fn send_message_to(&mut self, receiver: &str, message: &Message) -> io::Result<()> {
        if self.database.contains_client(receiver) {
            if !self.send_message_to_client(receiver, message) {
                self.disconnected_error(receiver)?;
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

    pub fn send_message_to_client(&self, client: &str, message: &Message) -> bool {
        let stream_ref = match self.database.get_stream(client) {
            Some(stream_ref) => stream_ref,
            None => return false,
        };

        let mut stream = stream_ref.lock().unwrap();
        message.send_to(stream.deref_mut()).is_ok()
    }
}
