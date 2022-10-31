use std::{io, ops::DerefMut};

use crate::{
    message::Message,
    server::{
        client_handler::{responses::replies::CommandResponse, ClientHandler},
        client_trait::ClientTrait,
    },
};

use crate::server::client_handler::responses::errors::ErrorReply;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn message_command_to_targets(
        &mut self,
        command: &str,
        targets: String,
        content: String,
    ) -> io::Result<()> {
        for target in targets.split(',') {
            if let Some(error) = self.assert_target_is_valid(target) {
                self.send_response_for_error(error)?;
                continue;
            }

            let nickname = self.registration.nickname().unwrap();
            let message = format!(":{nickname} {command} {target} :{content}");

            let message = Message::new(&message).unwrap();
            self.send_message_to_target(&message, target)?;

            // if command == PRIVMSG_COMMAND && self.database.contains_client(target) {
            //     self.away_response_for_client(target);
            // }
        }

        self.send_response_for_reply(CommandResponse::Ok)
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
