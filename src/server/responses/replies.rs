use std::fmt::Display;

use crate::server::data_structures::ClientInfo;

/// Possible s the commands can generate.
pub enum CommandResponse {
    WhoisUser311 {
        client_info: ClientInfo,
    },
    WhoisServer312 {
        nickname: String,
        server: String,
        server_info: String,
    },
    WhoisOperator313 {
        nickname: String,
    },
    EndOfWho315 {
        name: Option<String>,
    },
    // WhoisIdle317 {
    //     nickname: String,
    //     seconds: u8,
    // },
    EndOfWhois318 {
        nickname: String,
    },
    WhoisChannels319 {
        nickname: String,
        channels: Vec<String>,
    },
    ListStart321,
    List322 {
        channel: String,
        prv: bool,
        topic: String,
    },
    ListEnd323,
    NoTopic331 {
        channel: String,
    },
    Topic332 {
        channel: String,
        topic: String,
    },
    Inviting341 {
        channel: String,
        nickname: String,
    },
    WhoReply352 {
        channel: Option<String>,
        client_info: ClientInfo,
    },
    NameReply353 {
        channel: String,
        clients: Vec<String>,
    },
    EndOfNames366 {
        channel: String,
    },
    YouAreOper381,
    BanList367 {
        channel: String,
        banmask: String,
    },
    EndOfBanList368 {
        channel: String,
    },
    // ChannelModeIs324 {
    //     channel: String,
    //     mode: char,
    //     mode_params: Option<Vec<String>>,
    // },
    UnAway,
    NowAway,
    Away {
        nickname: String,
        message: String,
    },
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {

            CommandResponse::WhoisUser311 { client_info } => {
                format!(
                    "311 {} {} {} *: {}",
                    client_info.nickname,
                    client_info.username,
                    client_info.hostname,
                    client_info.realname,
                )
            }
            CommandResponse::WhoisServer312 {
                nickname,
                server,
                server_info,
            } => {
                format!("312 {nickname} {server} :{server_info}")
            }
            CommandResponse::WhoisOperator313 { nickname } => {
                format!("313 {nickname} :Is an IRC operator")
            }
            CommandResponse::EndOfWho315 { name } => {
                format!(
                    "315 {} :End of /WHO list",
                    name.to_owned().unwrap_or_default()
                )
            }
            // CommandResponse::WhoisIdle317 { nickname, seconds } => {
            //     format!("317 {nickname} {seconds} :seconds idle")
            // }
            CommandResponse::EndOfWhois318 { nickname } => {
                format!("318 {nickname} :End of /WHOIS list")
            }
            CommandResponse::WhoisChannels319 { nickname, channels } => {
                format!("319 {nickname} : {}", channels.join(" "))
            }
            CommandResponse::ListStart321 => "321 :Channel :Users Name".to_string(),
            CommandResponse::List322 {
                channel,
                topic,
                prv,
            } => {
                if *prv {
                    format!("322 {channel} Prv")
                } else {
                    format!("322 {channel} :{topic}")
                }
            }
            CommandResponse::ListEnd323 => "323 :End of /LIST".to_string(),
            CommandResponse::NoTopic331 { channel } => {
                format!("331 {channel} :No topic is set")
            }
            CommandResponse::Topic332 { channel, topic } => {
                format!("332 {} :{}", channel, topic)
            }
            CommandResponse::Inviting341 { channel, nickname } => {
                format!("341 {channel} {nickname}")
            }
            CommandResponse::WhoReply352 {
                channel,
                client_info,
            } => {
                format!(
                    "352 {} {} {} {} {} \\MODOS :HOPCOUNT {}",
                    channel.as_ref().unwrap_or(&"*".to_string()),
                    client_info.username,
                    client_info.hostname,
                    client_info.servername,
                    client_info.nickname,
                    client_info.realname,
                )
            }
            CommandResponse::NameReply353 { channel, clients } => {
                format!("353 {channel} :{}", clients.join(" "))
            }
            CommandResponse::EndOfNames366 { channel } => {
                format!("366 {channel} :End of /NAMES list")
            }
            CommandResponse::YouAreOper381 => "381 :You are now an IRC operator".to_string(),
            CommandResponse::BanList367 { channel, banmask } => {
                format!("367 {channel} {banmask}")
            }
            CommandResponse::EndOfBanList368 { channel } => {
                format!("368 {channel} :End of channel ban list")
            }
            // CommandResponse::ChannelModeIs324 {
            //     channel,
            //     mode,
            //     mode_params,
            // } => format!(
            //     "324 {channel} {mode} {:?}",
            //     mode_params.as_ref().unwrap_or(&vec!["".to_string()])
            // ),
            CommandResponse::UnAway => "305 :You are no longer marked as being away".to_string(),
            CommandResponse::NowAway => "306 :You have been marked as being away".to_string(),
            CommandResponse::Away { nickname, message } => {
                format!("301 {nickname} :{message}")
            }
        };
        write!(f, "{string}")
    }
}

impl CommandResponse {

    pub fn unaway() -> Self {
        Self::UnAway
    }

    pub fn now_away() -> Self {
        Self::NowAway
    }

    pub fn list_start() -> Self {
        Self::ListStart321
    }

    pub fn list_end() -> Self {
        Self::ListEnd323
    }

    pub fn you_are_oper() -> Self {
        Self::YouAreOper381
    }

    pub fn end_of_who(name: Option<String>) -> Self {
        Self::EndOfWho315 { name }
    }

    pub fn name_reply(channel: &str, clients: &[String]) -> Self {
        let channel = channel.to_string();
        let clients = clients.to_vec();
        Self::NameReply353 { channel, clients }
    }

    pub fn whois_user(client_info: &ClientInfo) -> Self {
        let client_info = client_info.clone();
        Self::WhoisUser311 { client_info }
    }

    pub fn end_of_whois(nickname: &str) -> Self {
        let nickname = nickname.to_string();
        Self::EndOfWhois318 { nickname }
    }

    pub fn whois_channel(nickname: &str, channels: &[String]) -> Self {
        let nickname = nickname.to_string();
        let channels = channels.to_vec();
        Self::WhoisChannels319 { nickname, channels }
    }

    pub fn whois_operator(nickname: &str) -> Self {
        let nickname = nickname.to_string();
        Self::WhoisOperator313 { nickname }
    }

    pub fn whois_server(nickname: &str, servername: &str, serverinfo: &str) -> Self {
        let nickname = nickname.to_string();
        let servername = servername.to_string();
        let serverinfo = serverinfo.to_string();
        Self::WhoisServer312 {
            nickname,
            server: servername,
            server_info: serverinfo,
        }
    }

    pub fn banlist(channel: &str, banmask: &str) -> Self {
        let channel = channel.to_string();
        let banmask = banmask.to_string();
        Self::BanList367 { channel, banmask }
    }

    pub fn end_of_banlist(channel: &str) -> Self {
        let channel = channel.to_string();
        Self::EndOfBanList368 { channel }
    }

    pub fn topic(channel: &str, topic: &str) -> Self {
        let channel = channel.to_string();
        let topic = topic.to_string();
        Self::Topic332 { channel, topic }
    }

    pub fn whoreply(channel: &Option<String>, client_info: &ClientInfo) -> Self {
        let channel = channel.clone();
        let client_info = client_info.clone();
        Self::WhoReply352 {
            channel,
            client_info,
        }
    }

    pub fn list(channel: String, topic: String, prv: bool) -> Self {
        Self::List322 {
            channel,
            prv,
            topic,
        }
    }

    pub fn inviting(inviting_client: String, channel: String) -> Self {
        Self::Inviting341 {
            nickname: inviting_client,
            channel,
        }
    }

    pub fn end_of_names(channel: &str) -> Self {
        Self::EndOfNames366 {
            channel: channel.to_string(),
        }
    }

    pub fn away(client: &str, message: &str) -> Self {
        let nickname = client.to_string();
        Self::Away {
            nickname,
            message: message.to_string(),
        }
    }

    pub fn no_topic(channel: &str) -> Self {
        let channel = channel.to_string();
        Self::NoTopic331 { channel }
    }
}
