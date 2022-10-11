use super::ParsingError;
use super::COLON;

use std::iter::Peekable;
use std::str::SplitWhitespace;

type Prefix = Option<String>;
type Command = String;
type Parameters = Vec<String>;
type Trailing = Option<String>;
type MessageParse = (Prefix, Command, Parameters, Trailing);

pub fn parse(content: &str) -> Result<MessageParse, ParsingError> {
    if content.is_empty() {
        return Err(ParsingError::EmptyMessage);
    }

    let mut words = content.split_whitespace().peekable();

    let prefix = get_prefix(&mut words)?;
    let command = get_command(&mut words)?;
    let parameters = get_parameters(&mut words);
    let trailing = get_trailing(&mut words);

    Ok((prefix, command, parameters, trailing))
}

fn get_prefix(split: &mut Peekable<SplitWhitespace>) -> Result<Prefix, ParsingError> {
    let possible_prefix = match split.peek() {
        None => return Err(ParsingError::EmptyMessage),
        Some(possible_prefix) => possible_prefix,
    };

    let first_character = *possible_prefix
        .as_bytes()
        .first()
        .expect("SplitWhitespace does not generate empty elements");

    if first_character == COLON {
        let prefix = split.next().expect("Existance was verified on peek");

        if prefix.len() == 1 {
            return Err(ParsingError::EmptyPrefix);
        }

        let prefix = &prefix[1..];
        return Ok(Some(prefix.to_string()));
    }

    Ok(None)
}
fn get_command(split: &mut Peekable<SplitWhitespace>) -> Result<Command, ParsingError> {
    let possible_command = match split.next() {
        None => return Err(ParsingError::NoCommand),
        Some(possible_command) => possible_command,
    };

    Ok(possible_command.to_string())
}
fn get_parameters(split: &mut Peekable<SplitWhitespace>) -> Parameters {
    let mut parameters = Vec::new();

    while let Some(possible_parameter) = split.peek() {
        let first_character = *possible_parameter
            .as_bytes()
            .first()
            .expect("SplitWhitespace does not generate empty elements");

        if first_character == COLON {
            return parameters;
        }

        let parameter = split.next().expect("Existance was verified on peek");
        parameters.push(parameter.to_string());
    }

    parameters
}
fn get_trailing(split: &mut Peekable<SplitWhitespace>) -> Trailing {
    split.peek()?;

    let string_list: Vec<String> = split.map(|word| word.to_string()).collect();
    let mut joined_string = string_list.join(" ");

    joined_string.remove(0);

    Some(joined_string)
}
