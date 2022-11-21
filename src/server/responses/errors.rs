use std::fmt::Display;
/// Possible errors the commands may have.
pub enum ErrorReply {
    NoSuchNickname401 { nickname: String },
    NoSuchChannel403 { channel: String },
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
    // InviteOnlyChannel473 { channel: String },
    BadChannelKey475 { channel: String },
    NoNickname,
    NotRegistered451,
    ParsingError,
    // NoSuchServer { server: String},
    UnknownMode472 { mode: char },
    KeySet467 { channel: String },
    ChanOPrivIsNeeded482 { channel: String },
    CannotSendToChannel404 { channel: String },
    ChannelIsFull471 { channel: String },
    BannedFromChannel474 { channel: String },
    NoReply,
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
            ErrorReply::TooManyChannels405 { channel } => {
                format!("405 {channel} :You have joined too many channels")
            }
            ErrorReply::NoRecipient411 { command } => {
                format!("411 :No recipient given ({command})")
            }
            ErrorReply::NoTextToSend412 => String::from("412 :No text to send"),
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
            ErrorReply::NoNickname => "200 :No nickname registered".to_string(),
            ErrorReply::NotRegistered451 => "451 :You have not registered".to_string(),
            ErrorReply::ParsingError => "200 :Parsing error".to_string(),
            ErrorReply::UnknownMode472 { mode } => {
                format!("472 {mode} :Is unknown mode char to me")
            }
            ErrorReply::KeySet467 { channel } => {
                format!("467 {channel} :Channel key already set")
            }
            ErrorReply::ChanOPrivIsNeeded482 { channel } => {
                format!("482 {channel} :You're not channel operator")
            }
            ErrorReply::CannotSendToChannel404 { channel } => {
                format!("404 {channel} :Cannot send to channel")
            }
            ErrorReply::BadChannelKey475 { channel } => {
                format!("475 {channel} :Cannot join channel (+k)")
            }
            ErrorReply::ChannelIsFull471 { channel } => {
                format!("471 {channel} :Cannot join channel (+l)")
            }
            ErrorReply::BannedFromChannel474 { channel } => {
                format!("474 {channel} :Cannot join channel (+b)")
            }
            ErrorReply::NoReply => "".to_string(),
        };
        write!(f, "{string}")
    }
}
