use crate::server::client_handler::commands::connection_registration::QUIT_COMMAND;
use std::fmt::Display;

pub enum CommandResponse {
    Ok200,
    EndOfWho315 {
        name: String,
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
        client: String,
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
                format!("315 {name} :End of /WHO list")
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
            CommandResponse::WhoReply352 { client } => {
                format!("352 :{client}")
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
            CommandResponse::Ok200 => "200 :success".to_string(),
            CommandResponse::ListStart321 => "321 :Channel :Users Name".to_string(),
            CommandResponse::ListEnd323 => "323 :End of /LIST".to_string(),
            CommandResponse::YouAreOper381 => "381 :You are now an IRC operator".to_string(),
            // CommandResponse::Away301 { nickname, message } => {
            //     format!("301 {nickname} :{message}")
            // }
            // CommandResponse::Topic332 { channel, topic } => {
            //     format!("332 {} :{}", channel, topic)
            // }
        };
        write!(f, "{string}")
    }
}
