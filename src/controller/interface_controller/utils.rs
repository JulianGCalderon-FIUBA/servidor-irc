use std::{collections::HashMap, net::TcpStream};

use crate::{
    controller::{
        utils::{
            all_clients, client_channels, push_if_absent, remove_element,
            remove_operator_indicator, OPERATOR_CHARACTER,
        },
        NAMES_ERROR_TEXT, OPEN_WARNING_ERROR_TEXT,
    },
    ctcp::dcc_message::DccMessage::{
        self, Accept, Chat, ChatAccept, ChatDecline, Close, Resume, Send, SendAccept, SendDecline,
    },
    message::Message,
    server::consts::commands::NAMES_COMMAND,
};

use super::{names_message_intention::NamesMessageIntention, InterfaceController};

use crate::controller::controller_message::ControllerMessage::OpenWarningView;

impl InterfaceController {
    /// Returns all clients to add.
    ///
    /// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
    pub fn all_clients_except_me(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let mut all_clients = all_clients(channels_and_clients);
        self.remove_myself(&mut all_clients);
        all_clients
    }

    /// Returns clients that are not from the current user.
    ///
    /// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
    pub fn clients_not_mine(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let all_clients = self.all_clients_except_me(channels_and_clients);
        let my_clients = self.main_view.get_my_clients();
        let mut not_mine: Vec<String> = vec![];

        for element in &all_clients {
            let element_without_operator_indicator: String = remove_operator_indicator(element);
            push_if_absent(
                &my_clients,
                &mut not_mine,
                element_without_operator_indicator,
            );
        }

        not_mine
    }

    /// Returns my current channels.
    ///
    /// Receives a hash map, returns a string vector with my channels.
    pub fn current_conv_channels(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        client_channels(channels_and_clients, self.current_conv.clone())
    }

    /// Get stream from client.
    ///
    /// Returns TcpStream.
    pub fn get_stream(&mut self) -> TcpStream {
        self.client.get_stream().unwrap()
    }

    /// Handles dcc message reception.
    pub fn receive_dcc_message(&mut self, message: Message, content: String) {
        let (_, _, sender_nickname) = self.decode_priv_message(message);

        let dcc_message = if let Ok(dcc_message) = DccMessage::parse(content) {
            dcc_message
        } else {
            return;
        };

        match dcc_message {
            Accept {
                filename,
                port,
                position,
            } => self.receive_dcc_accept(sender_nickname, filename, port, position),
            Chat { address } => {
                self.open_dcc_invitation_view(sender_nickname, address);
            }
            ChatAccept => {
                self.dcc_receive_accept(sender_nickname);
            }
            ChatDecline => {
                self.dcc_receive_decline(sender_nickname);
            }
            Close => {
                self.receive_dcc_close(sender_nickname);
            }
            Resume {
                filename,
                port,
                position,
            } => self.receive_dcc_resume(sender_nickname, filename, port, position),
            Send {
                filename,
                address,
                filesize,
            } => {
                self.receive_dcc_send(sender_nickname, filename, address, filesize);
            }
            SendAccept => {
                self.receive_dcc_send_accept(sender_nickname);
            }
            SendDecline => {
                self.receive_dcc_send_decline(sender_nickname);
            }
        }
    }

    /// Handles reception of a private message.
    pub fn receive_regular_privmsg(&mut self, message: Message) {
        let (channel, message, sender_nickname) = self.decode_priv_message(message);
        if let Some(..) = channel {
            self.main_view
                .receive_priv_channel_message(message, sender_nickname, channel.unwrap());
        } else {
            self.main_view
                .receive_priv_client_message(message, sender_nickname);
        }
    }

    /// Removes myself from clients list.
    pub fn remove_myself(&mut self, all_clients: &mut Vec<String>) {
        let nickname = self.nickname.clone();
        remove_element(all_clients, &nickname);
        remove_element(all_clients, &format!("{}{nickname}", OPERATOR_CHARACTER));
    }

    /// Sends names message to the server.
    pub fn send_names_message(
        &mut self,
        intention: NamesMessageIntention,
        parameter: Option<String>,
    ) {
        self.names_message_intention = intention;
        let parameter_value = parameter.unwrap_or_default();
        let message = format!("{NAMES_COMMAND} {}", parameter_value);
        self.client.send(&message).expect(NAMES_ERROR_TEXT);
    }

    /// Sends open warning controller message.
    pub fn send_open_warning_view(&mut self, warning_text: &str) {
        let to_send = OpenWarningView {
            message: warning_text.to_string(),
        };
        self.sender.send(to_send).expect(OPEN_WARNING_ERROR_TEXT);
    }
}
