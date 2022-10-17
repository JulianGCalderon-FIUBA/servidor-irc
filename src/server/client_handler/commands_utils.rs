fn is_numeric(a_value: &str) -> bool {
    a_value.chars().all(char::is_numeric)
}

fn is_positive_numeric(a_value: &str) -> bool {
    is_numeric(a_value) && a_value.parse::<isize>().unwrap() >= 0
}

fn there_is_trailing(trailing: &Option<String>) -> bool {
    trailing.is_some()
}

pub fn pass_command_is_valid(parameters: &Vec<String>, trailing: &Option<String>) -> bool {
    parameters.len() == 1 && !(there_is_trailing(trailing))
}

pub fn nick_command_is_valid(parameters: &Vec<String>, trailing: &Option<String>) -> bool {
    (parameters.len() == 1 || (parameters.len() == 2 && is_positive_numeric(&(parameters[1])[..])))
        && !there_is_trailing(trailing)
}

pub fn user_command_is_valid(parameters: &Vec<String>, trailing: &Option<String>) -> bool {
    parameters.len() == 3 && there_is_trailing(trailing)
}

pub fn quit_command_is_valid(parameters: &Vec<String>) -> bool {
    parameters.is_empty()
}

pub fn get_trailing(trailing: &Option<String>) -> &String {
    trailing.as_ref().unwrap()
}
