use super::format_error;
use super::COLON;
use std::io;
use std::iter::Peekable;
use std::str::SplitWhitespace;

type Prefix = Option<String>;
type Command = String;
type Parameters = Vec<String>;
type Trailing = Option<String>;
type MessageParse = (Prefix, Command, Parameters, Trailing);

pub fn parse(content: &str) -> io::Result<MessageParse> {
    if content.is_empty() {
        return Err(format_error());
    }

    let mut words = content.split_whitespace().peekable();

    let prefix = get_prefix(&mut words)?;
    let command = get_command(&mut words)?;
    let parameters = get_parameters(&mut words);
    let trailing = get_trailing(&mut words);

    Ok((prefix, command, parameters, trailing))
}

fn get_prefix(split: &mut Peekable<SplitWhitespace>) -> io::Result<Prefix> {
    let possible_prefix = match split.peek() {
        None => return Err(format_error()),
        Some(possible_prefix) => possible_prefix,
    };

    if *possible_prefix.as_bytes().first().unwrap() == COLON {
        let prefix = split.next().unwrap();

        if prefix.len() > 1 {
            let prefix = &prefix[1..];
            return Ok(Some(prefix.to_string()));
        }
        return Err(format_error());
    }

    Ok(None)
}
fn get_command(split: &mut Peekable<SplitWhitespace>) -> io::Result<Command> {
    let possible_command = match split.next() {
        None => return Err(format_error()),
        Some(possible_command) => possible_command,
    };

    Ok(possible_command.to_string())
}
fn get_parameters(split: &mut Peekable<SplitWhitespace>) -> Parameters {
    let mut parameters = Vec::new();

    while let Some(possible_parameter) = split.peek() {
        if *possible_parameter.as_bytes().first().unwrap() == COLON {
            return parameters;
        }
        let parameter = split.next().unwrap();

        parameters.push(parameter.to_string());
    }

    parameters
}
fn get_trailing(split: &mut Peekable<SplitWhitespace>) -> Trailing {
    split.peek()?;

    let string_list: Vec<String> = split.map(|word| word.to_string()).collect();
    let mut joined_string = string_list.join(" ");

    joined_string.remove(0);

    if joined_string.is_empty() {
        return None;
    }

    Some(joined_string)
}
