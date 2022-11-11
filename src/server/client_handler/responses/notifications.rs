use std::fmt::Display;

use crate::server::client_handler::commands::{
    INVITE_COMMAND, KICK_COMMAND, NOTICE_COMMAND, PRIVMSG_COMMAND, QUIT_COMMAND,
};
/// Possible notifications that can be sent for different commands.
pub enum Notification {
    Quit {
        message: String,
    },
    Invite {
        inviting_client: String,
        invited_client: String,
        channel: String,
    },
    Privmsg {
        prefix: String,
        target: String,
        message: String,
    },
    Notice {
        prefix: String,
        target: String,
        message: String,
    },
    Kick {
        kicker: String,
        channel: String,
        nickname: String,
        comment: Option<String>,
    },
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Notification::Quit { message } => {
                format!("{QUIT_COMMAND} :{message}")
            }
            Notification::Invite {
                inviting_client,
                invited_client,
                channel,
            } => {
                format!(":{inviting_client} {INVITE_COMMAND} {invited_client} {channel}")
            }
            Notification::Privmsg {
                prefix,
                target,
                message,
            } => {
                format!(":{prefix} {PRIVMSG_COMMAND} {target} :{message}")
            }
            Notification::Notice {
                prefix,
                target,
                message,
            } => {
                format!(":{prefix} {NOTICE_COMMAND} {target} :{message}")
            }
            Notification::Kick {
                kicker,
                channel,
                nickname,
                comment,
            } => match comment {
                Some(comment) => {
                    format!(":{kicker} {KICK_COMMAND} {channel} {nickname} :{comment}")
                }
                None => format!(":{kicker} {KICK_COMMAND} {channel} {nickname}"),
            },
        };

        write!(f, "{string}")
    }
}
