use super::ClientHandler;
use crate::message::Message;
use crate::server::client_handler::commands_utils::{
    get_trailing, nick_command_is_valid, pass_command_is_valid, quit_command_is_valid,
    user_command_is_valid,
};
use std::io;

pub const PASS_COMMAND: &str = "PASS";
pub const NICK_COMMAND: &str = "NICK";
pub const USER_COMMAND: &str = "USER";
pub const QUIT_COMMAND: &str = "QUIT";

const MENSAJE_COMANDO_ENVIADO: &str = "Comando enviado!";
const MENSAJE_ERROR_ENVIO_COMANDO: &str = "Error en el envío del comando :(";
// use super::Message;

impl<'a> ClientHandler<'a> {
    pub fn execute_pass_command(&mut self, parameters: &[String]) {
        self.client.password = Some(parameters[0].clone());
    }

    pub fn execute_nick_command(&mut self, parameters: &[String]) {
        self.client.nickname = Some(parameters[0].clone());
    }

    pub fn execute_user_command(&mut self, parameters: &[String], trailing: &Option<String>) {
        self.client.username = Some(parameters[0].clone());
        self.client.hostname = Some(parameters[1].clone());
        self.client.servername = Some(parameters[2].clone());
        self.client.realname = trailing.clone();
    }

    pub fn pass_command(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> io::Result<()> {
        let message = if pass_command_is_valid(parameters, trailing) {
            self.execute_pass_command(parameters);
            Message::new(MENSAJE_COMANDO_ENVIADO).unwrap()
        } else {
            Message::new(MENSAJE_ERROR_ENVIO_COMANDO).unwrap()
        };
        message.send_to(&mut self.client.stream)
    }

    pub fn nick_command(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> io::Result<()> {
        let message = if nick_command_is_valid(parameters, trailing) {
            self.execute_nick_command(parameters);
            Message::new(MENSAJE_COMANDO_ENVIADO).unwrap()
        } else {
            Message::new(MENSAJE_ERROR_ENVIO_COMANDO).unwrap()
        };
        message.send_to(&mut self.client.stream)
    }

    pub fn user_command(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> io::Result<()> {
        let message = if user_command_is_valid(parameters, trailing) {
            self.execute_user_command(parameters, trailing);
            Message::new(MENSAJE_COMANDO_ENVIADO).unwrap()
        } else {
            Message::new(MENSAJE_ERROR_ENVIO_COMANDO).unwrap()
        };
        message.send_to(&mut self.client.stream)
    }

    pub fn quit_command(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> io::Result<()> {
        let message = if quit_command_is_valid(parameters) {
            if trailing.is_some() {
                Message::new(get_trailing(trailing)).unwrap()
            } else {
                Message::new(MENSAJE_COMANDO_ENVIADO).unwrap()
            }
        } else {
            Message::new("Error en el comando pass").unwrap()
        };
        message.send_to(&mut self.client.stream)
    }
}
