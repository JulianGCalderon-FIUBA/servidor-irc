fn is_numeric(&mut self, a_value: &str) -> bool {
    a_value.chars().all(char::is_numeric)
}

fn is_positive_numeric(&mut self, a_value: &str) -> bool {
    self.is_numeric(a_value) && a_value.parse::<isize>().unwrap() >= 0
}

fn there_is_trailing(&mut self, trailing: &Option<String>) -> bool {
    trailing.is_some()
}

pub fn pass_command_is_valid(
    &mut self,
    parameters: &Vec<String>,
    trailing: &Option<String>,
) -> bool {
    parameters.len() == 1 && !(self.there_is_trailing(trailing))
}

pub fn nick_command_is_valid(
    &mut self,
    parameters: &Vec<String>,
    trailing: &Option<String>,
) -> bool {
    (parameters.len() == 1
        || (parameters.len() == 2 && self.is_positive_numeric(&(parameters[1])[..])))
        && !self.there_is_trailing(trailing)
}

pub fn user_command_is_valid(
    &mut self,
    parameters: &Vec<String>,
    trailing: &Option<String>,
) -> bool {
    parameters.len() == 3 && self.there_is_trailing(trailing)
}