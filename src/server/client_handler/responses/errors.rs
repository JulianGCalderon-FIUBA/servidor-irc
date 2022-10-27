use std::{fmt::Display, io};

use crate::server::{client_handler::ClientHandler, client_trait::ClientTrait};

pub enum ErrorReply {
    NoSuchNickname401 { nickname: String },
    NoSuchChannel403 { channel: String },
    CanNotSendToChannel404 { channel: String },
    TooManyChannels405 { channel: String },
    NoRecipient411 { command: String },
    NoTextToSend412,
    UnknownCommand421 { command: String },
    NoNicknameGiven431,
    NotOnChannel442 { channel: String },
    UserOnChannel443 { nickname: String, channel: String },
    NeedMoreParameters461 { command: String },
    PasswordMismatch464,
    NoNickname,
    UnregisteredClient,
    ClientOffline { nickname: String },
    // ChannelIsFull471 { channel: String },
    // InviteOnlyChannel473 { channel: String },
    // BannedFromChannel474 { channel: String },
    // BadChannelKey475 { channel: String },
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
            ErrorReply::ClientOffline { nickname } => {
                format!("200 {nickname} :client is offline")
            }
            ErrorReply::NoNicknameGiven431 => "431 :no nickname given".to_string(),
            ErrorReply::NoTextToSend412 => "412 :no text to send".to_string(),
            ErrorReply::PasswordMismatch464 => "464 :password incorrect".to_string(),
            ErrorReply::NoNickname => "200 :no nickname registered".to_string(),
            ErrorReply::UnregisteredClient => "200 :unregistered".to_string(),
        };
        write!(f, "{string}")
    }
}

impl<T: ClientTrait> ClientHandler<T> {
    // REPLY o ERROR

    pub fn no_nickname_error(&mut self) -> io::Result<()> {
        let response = "200 :no nickname registered".to_string();
        self.send_response(&response)
    }

    pub fn unregistered_error(&mut self) -> io::Result<()> {
        let response = "200 :unregistered".to_string();
        self.send_response(&response)
    }

    pub fn no_such_nickname_error(&mut self, nickname: &str) -> io::Result<()> {
        let response = format!("401 {nickname} :No such nick/channel");
        self.send_response(&response)
    }

    pub fn no_such_channel_error(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("403 {channel} :no such channel");
        self.send_response(&response)
    }

    pub fn cannot_send_to_chan_error(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("404 {channel} :cannot send to channel");
        self.send_response(&response)
    }

    pub fn too_many_channels_error(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("405 {channel} :you have joined too many channels");
        self.send_response(&response)
    }

    pub fn no_recipient_error(&mut self, command: &str) -> io::Result<()> {
        let response = format!("411 :no recipient given ({command})");
        self.send_response(&response)
    }

    pub fn no_text_to_send_error(&mut self) -> io::Result<()> {
        let response = "412 :no text to send".to_string();
        self.send_response(&response)
    }

    pub fn unknown_command_error(&mut self, command: &str) -> io::Result<()> {
        let response = format!("421 {command} :unknown command");
        self.send_response(&response)
    }

    pub fn no_nickname_given_error(&mut self) -> io::Result<()> {
        let response = "431 :no nickname given".to_string();
        self.send_response(&response)
    }

    pub fn not_on_channel_error(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("442 {channel} :you're not on that channel");
        self.send_response(&response)
    }

    pub fn user_on_channel_error(&mut self, user: &str, channel: &str) -> io::Result<()> {
        let response = format!("443 {user} {channel} :is already on channel");
        self.send_response(&response)
    }

    pub fn need_more_params_error(&mut self, command: &str) -> io::Result<()> {
        let response = format!("461 {command} :not enough parameters");
        self.send_response(&response)
    }

    // pub fn password_mismatch_error(&mut self) -> io::Result<()> {
    //     let response = "464 :password incorrect".to_string();
    //     self.send_response(&response)
    // }

    // pub fn channel_is_full_error(&mut self, channel: &str) -> io::Result<()> {
    //     let response = format!("471 {} :cannot join channel (+l)", channel);
    //     self.send_response(&response)
    // }

    // pub fn invite_only_channel_error(&mut self, channel: &str) -> io::Result<()> {
    //     let response = format!("473 {} :cannot join channel (+i)", channel);
    //     self.send_response(&response)
    // }

    // pub fn banned_from_channel_error(&mut self, channel: &str) -> io::Result<()> {
    //     let response = format!("474 {} :cannot join channel (+b)", channel);
    //     self.send_response(&response)
    // }

    // pub fn bad_channel_key_error(&mut self, channel: &str) -> io::Result<()> {
    //     let response = format!("475 {} :cannot join channel (+k)", channel);
    //     self.send_response(&response)
    // }

    pub fn disconnected_error(&mut self, nickname: &str) -> io::Result<()> {
        let response = format!("200 {nickname} :client is offline");
        self.send_response(&response)
    }
}
