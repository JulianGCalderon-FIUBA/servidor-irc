use std::{io, ops::DerefMut};

use crate::{message::Message, server::client_handler::ClientHandler};

fn _is_numeric(a_value: &str) -> bool {
    a_value.chars().all(char::is_numeric)
}

fn _is_positive_numeric(a_value: &str) -> bool {
    _is_numeric(a_value) && a_value.parse::<isize>().unwrap() >= 0
}

impl ClientHandler {
    pub fn build_text_message(&self, command: &str, receiver: &str, content: &str) -> Message {
        let message = format!(
            ":{} {} {} :{}",
            self.connection.nickname.as_ref().unwrap(),
            command,
            receiver,
            content
        );

        Message::new(&message).unwrap()
    }

    pub fn send_message_to(&mut self, receiver: &str, message: &Message) -> io::Result<()> {
        if self.database.contains_client(receiver) {
            if self.send_message_to_client(receiver, message).is_err() {
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
            if self.send_message_to_client(&client, message).is_err() {
                eprintln!("{} is offline", client);
            };
        }
    }

    pub fn send_message_to_client(&self, client: &str, message: &Message) -> io::Result<()> {
        let stream_ref = self.database.get_stream(client).unwrap();
        let mut stream = stream_ref.lock().unwrap();
        message.send_to(stream.deref_mut())
    }
}
