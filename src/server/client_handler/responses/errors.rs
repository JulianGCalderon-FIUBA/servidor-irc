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
    NoNickname,
    UnregisteredClient,
    ClientOffline { nickname: String },
    // ChannelIsFull471 { channel: String },
    // InviteOnlyChannel473 { channel: String },
    // BannedFromChannel474 { channel: String },
    // BadChannelKey475 { channel: String },
    //PasswordMismatch464,
}

impl Display for ErrorReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ErrorReply::NoSuchNickname401 { nickname } => {
                format!("401 {nickname} :No such nick/channel")
            }
            ErrorReply::NoSuchChannel403 { channel } => {
                format!("403 {channel} :no such channel")
            }
            ErrorReply::CanNotSendToChannel404 { channel } => {
                format!("404 {channel} :cannot send to channel")
            }
            ErrorReply::TooManyChannels405 { channel } => {
                format!("405 {channel} :you have joined too many channels")
            }
            ErrorReply::NoRecipient411 { command } => {
                format!("411 :no recipient given ({command})")
            }
            ErrorReply::UnknownCommand421 { command } => {
                format!("421 {command} :unknown command")
            }
            ErrorReply::NotOnChannel442 { channel } => {
                format!("442 {channel} :you're not on that channel")
            }
            ErrorReply::UserOnChannel443 { nickname, channel } => {
                format!("443 {nickname} {channel} :is already on channel")
            }
            ErrorReply::NeedMoreParameters461 { command } => {
                format!("461 {command} :not enough parameters")
            }
            ErrorReply::NicknameInUse433 { nickname } => {
                format!("433 {nickname} :nickname is already in use")
            }
            ErrorReply::NickCollision436 { nickname } => {
                format!("436 {nickname} :nickname collision KILL")
            }
            ErrorReply::ClientOffline { nickname } => {
                format!("200 {nickname} :client is offline")
            }
            ErrorReply::NoTextToSend412 => "412 :no text to send".to_string(),
            ErrorReply::NoNicknameGiven431 => "431 :no nickname given".to_string(),
            ErrorReply::AlreadyRegistered462 => "462 :you may not reregister".to_string(),
            ErrorReply::NoNickname => "200 :no nickname registered".to_string(),
            ErrorReply::UnregisteredClient => "200 :unregistered".to_string(),
            //ErrorReply::PasswordMismatch464 => "464 :password incorrect".to_string(),
        };
        write!(f, "{string}")
    }
}
