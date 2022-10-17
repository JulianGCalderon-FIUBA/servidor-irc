use crate::message;

use super::ClientHandler;
// use super::Message;

impl<'a> ClientHandler<'a> {
    // VALIDATIONS

    fn is_numeric(&mut self, a_value: &str) -> bool {
        a_value.chars().all(char::is_numeric)
    }

    fn is_positive_numeric(&mut self, a_value: &str) -> bool {
        self.is_numeric(a_value) && a_value.parse::<isize>().unwrap() >= 0
    }

    fn there_is_trailing(&mut self, trailing: &Option<String>) -> bool {
        trailing.is_some()
    }

    fn pass_command_is_valid(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> bool {
        parameters.len() == 1 && !(self.there_is_trailing(trailing))
    }

    fn nick_command_is_valid(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> bool {
        (parameters.len() == 1
            || (parameters.len() == 2 && self.is_positive_numeric(&(parameters[1])[..])))
            && !self.there_is_trailing(trailing)
    }

    fn user_command_is_valid(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> bool {
        parameters.len() == 3 && self.there_is_trailing(trailing)
    }

    //COMANDS

    pub fn pass_command(&mut self, parameters: &Vec<String>, trailing: &Option<String>) {
        if self.pass_command_is_valid(parameters, trailing) {
            self.client.password = Some(parameters[0].clone());
        }
    }

    pub fn nick_command(&mut self, parameters: &Vec<String>, trailing: &Option<String>) {
        if self.nick_command_is_valid(parameters, trailing) {
            self.client.nickname = Some(parameters[0].clone());
        }
    }

    pub fn user_command(&mut self, parameters: &Vec<String>, trailing: &Option<String>) {
        if self.user_command_is_valid(parameters, trailing) {
            self.client.username = Some(parameters[0].clone());
            self.client.hostname = Some(parameters[1].clone());
            self.client.servername = Some(parameters[2].clone());
            self.client.realname = trailing.clone();
        }
    }
}
