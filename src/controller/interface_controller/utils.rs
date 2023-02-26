use std::{ collections::HashMap, net::TcpStream };

use crate::{
    controller::{
        utils::{
            all_clients,
            client_channels,
            push_if_absent,
            remove_element,
            remove_operator_indicator,
        },
        NAMES_ERROR_TEXT,
        OPEN_WARNING_ERROR_TEXT,
    },
    ctcp::dcc_message::DccMessage,
    message::Message,
    server::consts::commands::NAMES_COMMAND,
};

use super::{ names_message_intention::NamesMessageIntention, InterfaceController };

use crate::controller::controller_message::ControllerMessage::OpenWarningView;

impl InterfaceController {
    /// Returns all clients to add.
    ///
    /// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
    pub fn all_clients_except_me(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>
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
        channels_and_clients: HashMap<String, Vec<String>>
    ) -> Vec<String> {
        let all_clients = self.all_clients_except_me(channels_and_clients);
        let my_clients = self.main_view.get_my_clients();
        let mut not_mine: Vec<String> = vec![];

        for element in &all_clients {
            let element_without_operator_indicator: String = remove_operator_indicator(element);
            push_if_absent(&my_clients, &mut not_mine, element_without_operator_indicator);
        }

        not_mine
    }

    pub fn current_conv_channels(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>
    ) -> Vec<String> {
        client_channels(channels_and_clients, self.current_conv.clone())
    }

    pub fn remove_myself(&mut self, all_clients: &mut Vec<String>) {
        let nickname = self.nickname.clone();
        remove_element(all_clients, &nickname);
        remove_element(all_clients, &format!("@{nickname}"));
    }

    pub fn send_names_message(
        &mut self,
        intention: NamesMessageIntention,
        parameter: Option<String>
    ) {
        self.names_message_intention = intention;
        let parameter_value = parameter.unwrap_or_default();
        let message = format!("{NAMES_COMMAND} {}", parameter_value);
        self.client.send(&message).expect(NAMES_ERROR_TEXT);
    }

    pub fn send_open_warning_view(&mut self, warning_text: &str) {
        let to_send = OpenWarningView {
            message: warning_text.to_string(),
        };
        self.sender.send(to_send).expect(OPEN_WARNING_ERROR_TEXT);
    }

    pub fn receive_regular_privmsg(&mut self, message: Message) {
        let (channel, message, sender_nickname) = self.decode_priv_message(message);
        if let Some(..) = channel {
            self.main_view.receive_priv_channel_message(message, sender_nickname, channel.unwrap());
        } else {
            self.main_view.receive_priv_client_message(message, sender_nickname);
        }
    }

    pub fn receive_dcc_message(&mut self, message: Message, content: String) {
        let (_, _, sender_nickname) = self.decode_priv_message(message);

        let dcc_message = if let Ok(dcc_message) = DccMessage::parse(content) {
            dcc_message
        } else {
            return;
        };

        match dcc_message {
            DccMessage::Send { filename, address, filesize } => {
                self.receive_dcc_send(sender_nickname, filename, address, filesize);
            }
            DccMessage::SendAccept => {
                self.receive_dcc_send_accept(sender_nickname);
            }
            DccMessage::SendDecline => {
                self.receive_dcc_send_decline(sender_nickname);
            }
            DccMessage::Chat { address } => {
                self.open_dcc_invitation_view(sender_nickname, address);
            }
            DccMessage::ChatAccept => {
                self.dcc_receive_accept(sender_nickname);
            }
            DccMessage::ChatDecline => {
                self.dcc_receive_decline(sender_nickname);
            }
            DccMessage::Close => todo!(),
            DccMessage::Resume { filename, port, position } =>
                self.receive_dcc_resume(sender_nickname, filename, port, position),
            DccMessage::Accept { filename, port, position } =>
                self.receive_dcc_accept(sender_nickname, filename, port, position),
        }
    }

    pub fn get_stream(&mut self) -> TcpStream {
        self.client.get_stream().unwrap()
    }
}