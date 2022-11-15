use std::io;

use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_handler::ClientHandler;
use crate::server::client_trait::Connection;
use crate::server::database::ClientInfo;

pub const OPERATOR_SYMBOL: char = '@';
pub const SPEAKER_SYMBOL: char = '+';

impl<C: Connection> ClientHandler<C> {
    /// Returns filtered list of clients.
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
    /// Sends full who reply.
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
    /// Sends full whois reply.
    pub fn send_whois_responses(&mut self, client_info: ClientInfo) -> Result<(), io::Error> {
        let nickname = client_info.nickname.clone();
        let server = self.servername.to_string();

        self.send_response_for_reply(CommandResponse::WhoisUser311 { client_info })?;
        self.send_response_for_reply(CommandResponse::WhoisServer312 {
            nickname: nickname.clone(),
            server,
            server_info: "Lemon pie server".to_string(),
        })?;
        if self.database.is_server_operator(&nickname) {
            self.send_response_for_reply(CommandResponse::WhoisOperator313 {
                nickname: nickname.to_string(),
            })?;
        }
        let mut channels = self.database.get_channels_for_client(&nickname);
        if !channels.is_empty() {
            for channel in &mut channels {
                if self.database.is_channel_operator(channel, &nickname) {
                    channel.insert(0, OPERATOR_SYMBOL);
                }
                if self.database.channel_has_mode(channel, 'm')
                    && self.database.is_channel_speaker(channel, &nickname)
                {
                    channel.insert(0, SPEAKER_SYMBOL);
                }
            }
            self.send_response_for_reply(CommandResponse::WhoisChannels319 {
                nickname: nickname.to_string(),
                channels,
            })?;
        }
        self.send_response_for_reply(CommandResponse::EndOfWhois318 { nickname })?;
        Ok(())
    }
}
