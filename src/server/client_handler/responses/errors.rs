use std::io::{self, Read, Write};

use crate::server::client_handler::ClientHandler;

impl<T: Read + Write> ClientHandler<T> {
    // REPLY o ERROR

    pub fn no_nickname_error(&mut self) -> io::Result<()> {
        let response = "200 :no nickname registered".to_string();
        self.send_response(&response)
    }

    pub fn unregistered_error(&mut self) -> io::Result<()> {
        let response = "300 :unregistered".to_string();
        self.send_response(&response)
    }

    pub fn no_such_nickname_error(&mut self, nickname: &str) -> io::Result<()> {
        let response = format!("401 {} :No such nick/channel", nickname);
        self.send_response(&response)
    }

    pub fn no_such_channel_error(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("403 {} :no such channel", channel);
        self.send_response(&response)
    }

    pub fn cannot_send_to_chan_error(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("404 {channel} :cannot send to channel");
        self.send_response(&response)
    }

    pub fn too_many_channels_error(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("405 {} :you have joined too many channels", channel);
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
        let response = format!("442 {} :you're not on that channel", channel);
        self.send_response(&response)
    }

    pub fn user_on_channel_error(&mut self, user: &str, channel: &str) -> io::Result<()> {
        let response = format!("443 {} {} :is already on channel", user, channel);
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
        let response = format!("300 {} :client is offline", nickname);
        self.send_response(&response)
    }
}
