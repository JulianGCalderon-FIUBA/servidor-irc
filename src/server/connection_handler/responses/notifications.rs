use std::fmt::Display;

use crate::server::connection_handler::commands::*;

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
    Part {
        nickname: String,
        channel: String,
    },
    Join {
        nickname: String,
        channel: String,
    },
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Notification::Quit { message } => {
                format!("{} :{}", QUIT_COMMAND, message)
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
            Notification::Part { nickname, channel } => {
                format!(":{nickname} {PART_COMMAND} {channel}")
            }
            Notification::Join { nickname, channel } => {
                format!(":{nickname} {JOIN_COMMAND} {channel}")
            }
        };

        write!(f, "{string}")
    }
}
