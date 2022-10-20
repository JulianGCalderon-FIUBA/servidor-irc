use std::io;

use super::{ClientHandler, QUIT_COMMAND};
use crate::message::Message;

impl ClientHandler {
    pub fn send_response(&mut self, response: &str) -> io::Result<()> {
        let response = Message::new(response).unwrap();
        response.send_to(&mut self.stream)
    }

    pub fn ok_reply(&mut self) -> io::Result<()> {
        let response = "300 :success".to_string();
        self.send_response(&response)
    }

    pub fn need_more_params_error(&mut self, command: &str) -> io::Result<()> {
        let response = format!("461 {} :not enough parameters", command);
        self.send_response(&response)
    }

    pub fn no_nickname_given_error(&mut self) -> io::Result<()> {
        let response = "431 :no nickname given".to_string();
        self.send_response(&response)
    }

    pub fn quit_reply(&mut self, message: &str) -> io::Result<()> {
        let response = format!("{} :{}", QUIT_COMMAND, message);
        self.send_response(&response)
    }

    pub fn unknown_command_error(&mut self, command: &str) -> io::Result<()> {
        let response = format!("421 {} :unknown command", command);
        self.send_response(&response)
    }

    pub fn nickname_collision_response(&mut self) -> io::Result<()> {
        let response = "436 :nickname collision KILL".to_string();
        self.send_response(&response)
    }

    // pub fn nickname_in_use_response(&mut self) -> io::Result<()> {
    //     let response = format!("433 :nickname is already in use");
    //     self.send_response(&response)
    // }

    pub fn already_registered_response(&mut self) -> io::Result<()> {
        let response = "462 :may not reregister".to_string();
        self.send_response(&response)
    }

    pub fn no_nickname_error(&mut self) -> io::Result<()> {
        let response = "300 :no nickname registered".to_string();
        self.send_response(&response)
    }

    pub fn unregistered_error(&mut self) -> io::Result<()> {
        let response = "300 :unregistered".to_string();
        self.send_response(&response)
    }

    pub fn no_such_channel_response(&mut self, channel: String) -> io::Result<()> {
        let response = "300 :no such channel ".to_string() + &channel;
        self.send_response(&response)
    }

    pub fn names_reply(&mut self, channel: String, clients: Vec<String>) -> io::Result<()> {
        let response = "300 :".to_string() + &channel + " :" + &clients.join(", ");
        self.send_response(&response)
    }
}
