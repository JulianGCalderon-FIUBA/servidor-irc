use std::fmt::Display;

use crate::server::database::ClientInfo;
/// Possible responses the commands can generate.
pub enum CommandResponse {
    // Away301 {
    //     nickname: String,
    //     message: String,
    // },
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
    ChannelModeIs324 {
        channel: String,
        mode: char,
        mode_params: Option<Vec<String>>,
    },
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            // CommandResponse::Away301 { nickname, message } => {
            //     format!("301 {nickname} :{message}")
            // }
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
            CommandResponse::List322 { channel, topic } => {
                format!("322 {channel} :{topic}")
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
            CommandResponse::ChannelModeIs324 {
                channel,
                mode,
                mode_params,
            } => format!(
                "324 {channel} {mode} {:?}",
                mode_params.as_ref().unwrap_or(&vec!["".to_string()])
            ),
        };
        write!(f, "{string}")
    }
}
