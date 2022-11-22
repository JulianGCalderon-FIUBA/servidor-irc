use std::{
    fmt::Display,
    io::{self, Write},
    net::TcpStream,
};

use crate::{
    message::Message,
    server::{
        data_structures::*,
        responses::{CommandResponse, Notification},
    },
};

pub trait ConnectionResponses: Write + Sized {
    fn send(&mut self, message: &dyn Display) -> io::Result<()> {
        if let Ok(message) = Message::new(&message.to_string()) {
            return message.send_to(self);
        }

        Ok(())
    }

    fn send_name_reply(&mut self, channel: &str, clients: &[String]) -> io::Result<()> {
        let response = CommandResponse::NameReply353 {
            channel: channel.to_string(),
            clients: clients.to_vec(),
        };
        self.send(&response)
    }

    fn send_whois_user(&mut self, client_info: &ClientInfo) -> io::Result<()> {
        let client_info = client_info.clone();
        let response = CommandResponse::WhoisUser311 { client_info };
        self.send(&response)
    }

    fn send_end_of_whois(&mut self, nickname: &str) -> io::Result<()> {
        let nickname = nickname.to_string();
        let response = CommandResponse::EndOfWhois318 { nickname };
        self.send(&response)
    }

    fn send_whois_channel(&mut self, nickname: &str, channels: &[String]) -> Result<(), io::Error> {
        let nickname = nickname.to_string();
        let channels = channels.to_vec();
        let response = CommandResponse::WhoisChannels319 { nickname, channels };
        self.send(&response)
    }

    fn send_whois_operator(&mut self, nickname: &str) -> Result<(), io::Error> {
        let nickname = nickname.to_string();
        let response = CommandResponse::WhoisOperator313 { nickname };
        self.send(&response)
    }

    fn send_whois_server(
        &mut self,
        nickname: &str,
        servername: &str,
        serverinfo: &str,
    ) -> Result<(), io::Error> {
        let nickname = nickname.to_string();
        let servername = servername.to_string();
        let serverinfo = serverinfo.to_string();
        let response = CommandResponse::WhoisServer312 {
            nickname,
            server: servername,
            server_info: serverinfo,
        };
        self.send(&response)
    }

    fn send_banlist(&mut self, channel: &str, banmask: &str) -> io::Result<()> {
        let channel = channel.to_string();
        let banmask = banmask.to_string();
        let response = CommandResponse::BanList367 { channel, banmask };
        self.send(&response)
    }

    fn send_end_of_banlist(&mut self, channel: &str) -> io::Result<()> {
        let channel = channel.to_string();
        let response = CommandResponse::EndOfBanList368 { channel };
        self.send(&response)
    }

    fn send_topic(&mut self, channel: &str, topic: &str) -> io::Result<()> {
        let channel = channel.to_string();
        let topic = topic.to_string();
        let response = CommandResponse::Topic332 { channel, topic };
        self.send(&response)
    }

    fn send_whoreply_response(
        &mut self,
        channel: &Option<String>,
        client_info: &ClientInfo,
    ) -> io::Result<()> {
        let channel = channel.clone();
        let client_info = client_info.clone();
        let response = CommandResponse::WhoReply352 {
            channel,
            client_info,
        };
        self.send(&response)
    }

    fn send_list(&mut self, channel: String, topic: String, prv: bool) -> io::Result<()> {
        let response = CommandResponse::List322 {
            channel,
            prv,
            topic,
        };
        self.send(&response)
    }

    fn send_inviting(&mut self, inviting_client: String, channel: String) -> Result<(), io::Error> {
        let response = CommandResponse::Inviting341 {
            nickname: inviting_client,
            channel,
        };
        self.send(&response)
    }

    fn send_end_of_names(&mut self, channel: &str) -> Result<(), io::Error> {
        let response = CommandResponse::EndOfNames366 {
            channel: channel.to_string(),
        };
        self.send(&response)
    }
    fn send_quit(&mut self, message: &str) -> io::Result<()> {
        let notification = Notification::Quit {
            message: message.to_string(),
        };
        self.send(&notification)
    }

    fn send_away(&mut self, client: &str, message: &str) -> Result<(), io::Error> {
        let nickname = client.to_string();
        let response = CommandResponse::Away {
            nickname,
            message: message.to_string(),
        };
        self.send(&response)
    }

    fn send_server(
        &mut self,
        servername: &str,
        hopcount: usize,
        serverinfo: &str,
    ) -> io::Result<()> {
        let servername = servername.to_string();
        let serverinfo = serverinfo.to_string();
        let notification = Notification::Server {
            servername,
            hopcount,
            serverinfo,
        };

        self.send(&notification)
    }

    fn send_no_topic(&mut self, channel: &str) -> io::Result<()> {
        let channel = channel.to_string();
        let reply = CommandResponse::NoTopic331 { channel };
        self.send(&reply)
    }
}

impl ConnectionResponses for TcpStream {}
