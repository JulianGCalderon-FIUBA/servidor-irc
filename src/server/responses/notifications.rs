use std::fmt::Display;

use crate::server::consts::commands::*;
use crate::server::data_structures::*;

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
        client: ClientInfo,
    },
    Server {
        servername: String,
        hopcount: usize,
        serverinfo: String,
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
            Notification::User { client } => {
                format!(
                    ":{} {USER_COMMAND} {} {} {} :{}",
                    client.nickname,
                    client.username,
                    client.hostname,
                    client.servername,
                    client.realname
                )
            }
            Notification::Server {
                servername,
                hopcount,
                serverinfo,
            } => {
                format!("{SERVER_COMMAND} {servername} {hopcount} :{serverinfo}")
            }
        };

        write!(f, "{string}")
    }
}

impl Notification {
    pub fn quit(message: &str) -> Self {
        Notification::Quit {
            message: message.to_string(),
        }
    }

    pub fn server(servername: &str, hopcount: usize, serverinfo: &str) -> Self {
        let servername = servername.to_string();
        let serverinfo = serverinfo.to_string();
        Notification::Server {
            servername,
            hopcount,
            serverinfo,
        }
    }

    pub fn invite(inviting_client: &str, invited_client: &str, channel: &str) -> Self {
        let inviting_client = inviting_client.to_string();
        let invited_client = invited_client.to_string();
        let channel = channel.to_string();

        Notification::Invite {
            inviting_client,
            invited_client,
            channel,
        }
    }

    pub fn user(client: &ClientInfo) -> Self {
        let client = client.clone();
        Notification::User { client }
    }

    pub fn nick(nickname: &str, hopcount: usize) -> Self {
        let nickname = nickname.to_string();
        Notification::Nick { nickname, hopcount }
    }

    pub fn privmsg(sender: &str, target: &str, message: &str) -> Self {
        let sender = sender.to_string();
        let target = target.to_string();
        let message = message.to_string();

        Notification::Privmsg {
            sender,
            target,
            message,
        }
    }

    pub fn notice(sender: &str, target: &str, message: &str) -> Self {
        let sender = sender.to_string();
        let target = target.to_string();
        let message = message.to_string();

        Notification::Notice {
            sender,
            target,
            message,
        }
    }

    pub fn kick(kicker: &str, channel: &str, kicked: &str, comment: &Option<String>) -> Self {
        let kicker = kicker.to_string();
        let channel = channel.to_string();
        let kicked = kicked.to_string();
        let comment = comment.clone();
        Notification::Kick {
            kicker,
            channel,
            kicked,
            comment,
        }
    }

    pub fn part(nickname: &str, channel: &str) -> Self {
        let nickname = nickname.to_string();
        let channel = channel.to_string();

        Notification::Part { nickname, channel }
    }

    pub fn join(nickname: &str, channel: &str) -> Self {
        let nickname = nickname.to_string();
        let channel = channel.to_string();

        Notification::Join { nickname, channel }
    }
}
