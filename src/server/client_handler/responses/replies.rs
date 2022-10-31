use crate::server::{
    client_handler::commands::connection_registration::QUIT_COMMAND, database::ClientInfo,
};
use std::fmt::Display;

pub enum CommandResponse {
    Ok200,
    EndOfWho315 {
        name: Option<String>,
    },
    WhoisUser311 {
        client_info: ClientInfo,
    },
    WhoisOperator313 {
        nickname: String,
    },
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
    },
    ListEnd323,
    NoTopic331 {
        channel: String,
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
    Quit {
        message: String,
    },
    // WhoisIdle317 {
    //     nickname: String,
    //     seconds: u8,
    // },
    // WhoisServer312 {
    //     nickname: String,
    //     server: String,
    //     server_info: String,
    // },
    // Away301 {
    //     nickname: String,
    //     message: String,
    // },
    // Topic332 {
    //     channel: String,
    //     topic: String,
    // },
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            CommandResponse::EndOfWho315 { name } => {
                if let Some(name) = name {
                    format!("315 {name} :End of /WHO list")
                } else {
                    "315 :End of /WHO list".to_string()
                }
            }
            CommandResponse::List322 { channel } => {
                format!("322 : {channel}")
            }
            CommandResponse::NoTopic331 { channel } => {
                format!("331 {channel} :no topic is set")
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
            CommandResponse::Quit { message } => {
                format!("{QUIT_COMMAND} :{message}")
            }
            CommandResponse::WhoisUser311 { client_info } => {
                format!(
                    "311 {} {} {} *: {}",
                    client_info.nickname,
                    client_info.username,
                    client_info.hostname,
                    client_info.realname,
                )
            }
            CommandResponse::Ok200 => "200 :success".to_string(),
            CommandResponse::ListStart321 => "321 :Channel :Users Name".to_string(),
            CommandResponse::ListEnd323 => "323 :End of /LIST".to_string(),
            CommandResponse::YouAreOper381 => "381 :You are now an IRC operator".to_string(),

            CommandResponse::WhoisOperator313 { nickname } => {
                format!("313 {nickname} :is an IRC operator")
            }
            CommandResponse::EndOfWhois318 { nickname } => {
                format!("318 {nickname} :End of /WHOIS list")
            }
            CommandResponse::WhoisChannels319 { nickname, channels } => {
                format!("319 {nickname} : {}", channels.join(" "))
            } // CommandResponse::WhoisServer312 {
              //     nickname,
              //     server,
              //     server_info,
              // } => {
              //     format!("312 {nickname} {server} :{server_info}")
              // }
              // CommandResponse::Away301 { nickname, message } => {
              //     format!("301 {nickname} :{message}")
              // }
              // CommandResponse::Topic332 { channel, topic } => {
              //     format!("332 {} :{}", channel, topic)
              // }
              // CommandResponse::WhoisIdle317 { nickname, seconds } => {
              //     format!("317 {nickname} {seconds} :seconds idle")
              // }
        };
        write!(f, "{string}")
    }
}
