use std::io;

use crate::server::{
    client_handler::{responses::replies::CommandResponse, ClientHandler},
    client_trait::ClientTrait,
    database::ClientInfo,
};

impl<T: ClientTrait> ClientHandler<T> {
    pub fn filtered_clients_for_default_who_command(
        &mut self,
        clients: Vec<ClientInfo>,
    ) -> Vec<ClientInfo> {
        clients
            .into_iter()
            .filter(|client_info| self.shares_channel_with_self(client_info))
            .collect()
    }

    fn shares_channel_with_self(&mut self, client_info: &ClientInfo) -> bool {
        let client_channels = self.database.get_channels_for_client(&client_info.nickname);
        let self_channels = self
            .database
            .get_channels_for_client(&self.registration.nickname().unwrap());

        !client_channels
            .iter()
            .any(|channel| self_channels.contains(channel))
    }

    pub fn send_whoreply_for_client(
        &mut self,
        client_info: crate::server::database::ClientInfo,
    ) -> io::Result<()> {
        let channel = self
            .database
            .get_channels_for_client(&client_info.nickname)
            .get(0)
            .map(|string| string.to_owned());

        self.send_response_for_reply(CommandResponse::WhoReply352 {
            channel,
            client_info,
        })
    }

    pub fn send_whois_responses(
        &mut self,
        client_info: ClientInfo,
        nick: &str,
        nickname: String,
    ) -> Result<(), io::Error> {
        self.send_response_for_reply(CommandResponse::WhoisUser311 { client_info })?;
        if self.database._is_server_operator(nick) {
            self.send_response_for_reply(CommandResponse::WhoisOperator313 { nickname })?;
        }
        let channels = self.database.get_channels_for_client(nick);
        if !channels.is_empty() {
            self.send_response_for_reply(CommandResponse::WhoisChannels319 {
                nickname: nick.to_string(),
                channels,
            })?;
        }
        self.send_response_for_reply(CommandResponse::EndOfWhois318 {
            nickname: nick.to_string(),
        })?;
        Ok(())
    }
}
