use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler, connection_handler_trait::ConnectionHandlerUtils,
        responses::CommandResponse,
    },
    database::ClientInfo,
};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn send_join_response(&mut self, channel: &str) -> io::Result<()> {
        self.send_topic_response(channel.to_string())?;

        let name_reply = &CommandResponse::NameReply353 {
            channel: channel.to_string(),
            clients: self.database.get_clients_for_channel(channel),
        };

        self.send_response(name_reply)
    }

    pub(super) fn send_whois_response(&mut self, client_info: ClientInfo) -> Result<(), io::Error> {
        let nickname = client_info.nickname.clone();
        let server = self.servername.to_string();

        self.send_response(&CommandResponse::WhoisUser311 { client_info })?;
        self.send_response(&CommandResponse::WhoisServer312 {
            nickname: nickname.clone(),
            server,
            server_info: "Lemon pie server".to_string(),
        })?;

        if self.database.is_server_operator(&nickname) {
            self.send_response(&CommandResponse::WhoisOperator313 {
                nickname: nickname.clone(),
            })?;
        }

        let mut channels = self.database.get_channels_for_client(&nickname);
        if !channels.is_empty() {
            self.append_channel_role(&mut channels, &nickname);
            self.send_response(&CommandResponse::WhoisChannels319 {
                nickname: nickname.clone(),
                channels,
            })?;
        }
        self.send_response(&CommandResponse::EndOfWhois318 { nickname })?;

        Ok(())
    }

    pub(super) fn send_banlist_response(&mut self, channel: &str) -> io::Result<()> {
        let bans = self.database.get_channel_banmask(channel);
        for b in bans {
            self.send_response(&CommandResponse::BanList367 {
                channel: channel.to_string(),
                banmask: b,
            })?;
        }
        self.send_response(&CommandResponse::EndOfBanList368 {
            channel: channel.to_string(),
        })?;
        Ok(())
    }

    pub(super) fn send_topic_response(&mut self, channel: String) -> Result<(), io::Error> {
        match self.database.get_topic_for_channel(&channel) {
            Some(topic) => self.send_response(&CommandResponse::Topic332 { channel, topic })?,
            None => self.send_response(&CommandResponse::NoTopic331 { channel })?,
        };
        Ok(())
    }

    pub(super) fn send_whoreply_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let channel = self
            .database
            .get_channels_for_client(&client_info.nickname)
            .get(0)
            .map(|string| string.to_owned());

        self.send_response(&CommandResponse::WhoReply352 {
            channel,
            client_info,
        })
    }
}
