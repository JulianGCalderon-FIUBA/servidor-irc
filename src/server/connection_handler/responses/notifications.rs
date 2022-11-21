use std::fmt::Display;

use crate::server::connection_handler::consts::commands::*;

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
        sender: String,
        target: String,
        message: String,
    },
    Notice {
        sender: String,
        target: String,
        message: String,
    },
    Kick {
        kicker: String,
        channel: String,
        kicked: String,
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
    Nick {
        nickname: String,
        hopcount: usize,
    },
    User {
        nickname: String,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
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
                sender: prefix,
                target,
                message,
            } => {
                format!(":{prefix} {PRIVMSG_COMMAND} {target} :{message}")
            }
            Notification::Notice {
                sender: prefix,
                target,
                message,
            } => {
                format!(":{prefix} {NOTICE_COMMAND} {target} :{message}")
            }
            Notification::Kick {
                kicker,
                channel,
                kicked,
                comment,
            } => match comment {
                Some(comment) => {
                    format!(":{kicker} {KICK_COMMAND} {channel} {kicked} :{comment}")
                }
                None => format!(":{kicker} {KICK_COMMAND} {channel} {kicked}"),
            },
            Notification::Part { nickname, channel } => {
                format!(":{nickname} {PART_COMMAND} {channel}")
            }
            Notification::Join { nickname, channel } => {
                format!(":{nickname} {JOIN_COMMAND} {channel}")
            }
            Notification::Nick { nickname, hopcount } => {
                format!("{NICK_COMMAND} {nickname} {hopcount}")
            }
            Notification::User {
                nickname,
                username,
                hostname,
                servername,
                realname,
            } => {
                format!(":{nickname} {USER_COMMAND} {username} {hostname} {servername} :{realname}")
            }
        };

        write!(f, "{string}")
    }
}
