use std::fmt::Display;

use crate::server::client_handler::commands::{
    INVITE_COMMAND, NOTICE_COMMAND, PRIVMSG_COMMAND, QUIT_COMMAND,
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
        };

        write!(f, "{string}")
    }
}
