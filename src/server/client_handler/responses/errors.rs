use std::fmt::Display;

pub enum ErrorReply {
    NoSuchNickname401 { nickname: String },
    NoSuchChannel403 { channel: String },
    CanNotSendToChannel404 { channel: String },
    TooManyChannels405 { channel: String },
    NoRecipient411 { command: String },
    NoTextToSend412,
    UnknownCommand421 { command: String },
    NoNicknameGiven431,
    NicknameInUse433 { nickname: String },
    NickCollision436 { nickname: String },
    NotOnChannel442 { channel: String },
    UserOnChannel443 { nickname: String, channel: String },
    NeedMoreParameters461 { command: String },
    AlreadyRegistered462,
    PasswordMismatch464,
    // ChannelIsFull471 { channel: String },
    // InviteOnlyChannel473 { channel: String },
    // BannedFromChannel474 { channel: String },
    // BadChannelKey475 { channel: String },
    //
    NoNickname,
    UnregisteredClient,
    ClientOffline { nickname: String },
    // NoSuchServer { server: String},
}

impl Display for ErrorReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ErrorReply::NoSuchNickname401 { nickname } => {
                format!("401 {nickname} :No such nick/channel")
            }
            ErrorReply::NoSuchChannel403 { channel } => {
                format!("403 {channel} :No such channel")
            }
            ErrorReply::CanNotSendToChannel404 { channel } => {
                format!("404 {channel} :Cannot send to channel")
            }
            ErrorReply::TooManyChannels405 { channel } => {
                format!("405 {channel} :You have joined too many channels")
            }
            ErrorReply::NoRecipient411 { command } => {
                format!("411 :No recipient given ({command})")
            }
            ErrorReply::NoTextToSend412 => "412 :No text to send".to_string(),
            ErrorReply::UnknownCommand421 { command } => {
                format!("421 {command} :Unknown command")
            }
            ErrorReply::NoNicknameGiven431 => "431 :No nickname given".to_string(),
            ErrorReply::NicknameInUse433 { nickname } => {
                format!("433 {nickname} :Nickname is already in use")
            }
            ErrorReply::NickCollision436 { nickname } => {
                format!("436 {nickname} :Nickname collision KILL")
            }
            ErrorReply::NotOnChannel442 { channel } => {
                format!("442 {channel} :You're not on that channel")
            }
            ErrorReply::UserOnChannel443 { nickname, channel } => {
                format!("443 {nickname} {channel} :Is already on channel")
            }
            ErrorReply::NeedMoreParameters461 { command } => {
                format!("461 {command} :Not enough parameters")
            }
            ErrorReply::AlreadyRegistered462 => "462 :You may not reregister".to_string(),
            ErrorReply::PasswordMismatch464 => "464 :Password incorrect".to_string(),
            ErrorReply::ClientOffline { nickname } => {
                format!("200 {nickname} :Client is offline")
            }
            ErrorReply::NoNickname => "200 :No nickname registered".to_string(),
            ErrorReply::UnregisteredClient => "200 :Unregistered".to_string(),
        };
        write!(f, "{string}")
    }
}
