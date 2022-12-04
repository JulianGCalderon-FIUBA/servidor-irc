use super::ParsingError;
use super::INVALID_CHARACTERS;
use super::MAX_LENGTH;
use super::PREFIX_CHARACTER;

use std::iter::Peekable;
use std::str::SplitWhitespace;

type Prefix = Option<String>;
type Command = String;
type Parameters = Vec<String>;
type Trailing = Option<String>;
type MessageParse = (Prefix, Command, Parameters, Trailing);

/// Parses string into prefix, command, parameters and trailing
pub fn parse(content: &str) -> Result<MessageParse, ParsingError> {
    if content.is_empty() {
        return Err(ParsingError::EmptyMessage);
    }
    if content.len() > MAX_LENGTH {
        return Err(ParsingError::TooManyParameters);
    }
    if content.contains(INVALID_CHARACTERS) {
        return Err(ParsingError::InvalidCharacter);
    }

    let mut words = content.split_whitespace().peekable();

    let prefix = get_prefix(&mut words)?;
    let command = get_command(&mut words)?;
    let parameters = get_parameters(&mut words)?;
    let trailing = get_trailing(&mut words)?;

    Ok((prefix, command, parameters, trailing))
}

/// If next iter item is a prefix, it consumes it and returns its value
fn get_prefix(split: &mut Peekable<SplitWhitespace>) -> Result<Prefix, ParsingError> {
    let possible_prefix = match split.peek() {
        None => return Err(ParsingError::EmptyMessage),
        Some(possible_prefix) => possible_prefix,
    };

    let first_character = *possible_prefix
        .as_bytes()
        .first()
        .expect("SplitWhitespace does not generate empty elements");

    if first_character == PREFIX_CHARACTER {
        let prefix = split.next().expect("Existance was verified on peek");

        if prefix.len() == 1 {
            return Err(ParsingError::EmptyPrefix);
        }

        let prefix = &prefix[1..];

        return Ok(Some(prefix.to_string()));
    }

    Ok(None)
}

/// If next iter item is a command, it consumes it and returns its value
fn get_command(split: &mut Peekable<SplitWhitespace>) -> Result<Command, ParsingError> {
    let possible_command = match split.next() {
        None => return Err(ParsingError::NoCommand),
        Some(possible_command) => possible_command,
    };

    Ok(possible_command.to_string())
}

/// Consumes parameters from iterator and returns them
fn get_parameters(split: &mut Peekable<SplitWhitespace>) -> Result<Parameters, ParsingError> {
    let mut parameters = Vec::new();

    while let Some(possible_parameter) = split.peek() {
        let first_character = *possible_parameter
            .as_bytes()
            .first()
            .expect("SplitWhitespace does not generate empty elements");

        if first_character == PREFIX_CHARACTER {
            break;
        }

        let parameter = split.next().expect("Existance was verified on peek");
        parameters.push(parameter.to_string());
    }

    if parameters.len() > 15 {
        return Err(ParsingError::TooManyParameters);
    }

    Ok(parameters)
}

/// If next iter item is a trailing parameter, it consumes it and returns its value
fn get_trailing(split: &mut Peekable<SplitWhitespace>) -> Result<Trailing, ParsingError> {
    if split.peek().is_none() {
        return Ok(None);
    }

    let string_list: Vec<String> = split.map(|word| word.to_string()).collect();
    let mut joined_string = string_list.join(" ");

    joined_string.remove(0);

    Ok(Some(joined_string))
}
