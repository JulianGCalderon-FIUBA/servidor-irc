use crate::server::client_handler::{
    commands::connection_registration::QUIT_COMMAND, ClientHandler,
};
use std::io::{self, Read, Write};

impl<T: Read + Write> ClientHandler<T> {
    pub fn nickname_in_use_reply(&mut self) -> io::Result<()> {
        let response = "433 :nickname is already in use".to_string();
        self.send_response(&response)
    }

    pub fn nickname_collision_reply(&mut self) -> io::Result<()> {
        let response = "436 :nickname collision KILL".to_string();
        self.send_response(&response)
    }

    pub fn already_registered_reply(&mut self) -> io::Result<()> {
        let response = "462 :may not reregister".to_string();
        self.send_response(&response)
    }

    pub fn quit_reply(&mut self, message: &str) -> io::Result<()> {
        let response = format!("{QUIT_COMMAND} :{message}");
        self.send_response(&response)
    }

    pub fn ok_reply(&mut self) -> io::Result<()> {
        let response = "200 :success".to_string();
        self.send_response(&response)
    }

    pub fn list_start_reply(&mut self) -> io::Result<()> {
        let response = "321 :Channel :Users Name".to_string();
        self.send_response(&response)
    }

    pub fn list_end_reply(&mut self) -> io::Result<()> {
        let response = "323 :End of /LIST".to_string();
        self.send_response(&response)
    }

    pub fn list_reply(&mut self, channels: Vec<String>) -> io::Result<()> {
        self.list_start_reply()?;
        let response = format!("322 : {}", channels.join(" "));
        self.send_response(&response)?;
        self.list_end_reply()
    }

    // pub fn away_reply(&mut self, nickname: &str, message: &str) -> io::Result<()> {
    //     let response = format!("301 {nickname} :{message}");
    //     self.send_response(&response)
    // }

    pub fn no_topic_reply(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("331 {channel} :no topic is set");
        self.send_response(&response)
    }

    // pub fn topic_reply(&mut self, channel: &str, topic: &str) -> io::Result<()> {
    //     let response = format!("332 {} :{}", channel, topic);
    //     self.send_response(&response)
    // }

    pub fn invite_reply(&mut self, channel: &str, nickname: &str) -> io::Result<()> {
        let response = format!("341 {channel} {nickname}");
        self.send_response(&response)
    }

    pub fn end_of_names_reply(&mut self, channel: &str) -> io::Result<()> {
        let response = format!("366 {channel} :End of /NAMES list");
        self.send_response(&response)
    }

    pub fn names_reply(&mut self, channel: &str, clients: Vec<String>) -> io::Result<()> {
        let response = format!("353 {channel} :{}", clients.join(" "));
        self.send_response(&response)
    }

    pub fn oper_reply(&mut self) -> io::Result<()> {
        let response = "381 :You are now an IRC operator".to_string();
        self.send_response(&response)
    }
}
