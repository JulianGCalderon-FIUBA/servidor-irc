use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::io::{Read, Write};
use std::iter::Peekable;
use std::str::SplitWhitespace;
use std::vec;

pub struct Message {
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
    trailing: Option<String>,
}

type Prefix = Option<String>;
type Command = String;
type Parameters = Vec<String>;
type Trailing = Option<String>;
type MessageParse = (Prefix, Command, Parameters, Trailing);

const CRLF: &[u8] = b"\r\n";
const COLON: u8 = b':';

impl Message {
    pub fn new(content: &str) -> io::Result<Self> {
        let (prefix, command, parameters, trailing) = parse(content)?;

        Ok(Self {
            prefix,
            command,
            parameters,
            trailing,
        })
    }

    pub fn send_to(&self, stream: &mut dyn Write) -> io::Result<()> {
        let content = self.to_string();
        let bytes = content.as_bytes();

        stream.write_all(bytes)?;
        stream.write_all(CRLF)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> io::Result<Self> {
        let mut reader = BufReader::new(stream);

        let mut content = String::new();

        let size = reader.read_line(&mut content)?;
        if size == 0 {
            return Err(format_error());
        }

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return Err(format_error());
        }

        Self::new(&content)
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, ":{} ", prefix)?;
        }

        write!(f, "{}", self.command)?;

        for parameter in self.parameters.iter() {
            write!(f, " {}", parameter)?;
        }

        if let Some(trailing) = &self.trailing {
            write!(f, " :{}", trailing)
        } else {
            Ok(())
        }
    }
}

fn parse(content: &str) -> io::Result<MessageParse> {
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

fn format_error() -> Error {
    Error::new(ErrorKind::InvalidInput, "invalid input")
}

#[cfg(test)]
mod tests_to_string {

    use super::*;

    #[test]
    fn only_command() {
        let message = Message {
            prefix: None,
            command: "COMMAND".to_string(),
            parameters: vec![],
            trailing: None,
        };

        let actual = message.to_string();
        let expected = "COMMAND";

        assert_eq!(&actual, expected);
    }

    #[test]
    fn with_prefix() {
        let message = Message {
            prefix: Some("prefix".to_string()),
            command: "COMMAND".to_string(),
            parameters: vec![],
            trailing: None,
        };

        let actual = message.to_string();
        let expected = ":prefix COMMAND";

        assert_eq!(&actual, expected);
    }

    #[test]
    fn with_one_parameter() {
        let message = Message {
            prefix: None,
            command: "COMMAND".to_string(),
            parameters: vec!["param1".to_string()],
            trailing: None,
        };

        let actual = message.to_string();
        let expected = "COMMAND param1";

        assert_eq!(&actual, expected);
    }

    #[test]
    fn with_two_parameters() {
        let message = Message {
            prefix: None,
            command: "COMMAND".to_string(),
            parameters: vec!["param1".to_string(), "param2".to_string()],
            trailing: None,
        };

        let actual = message.to_string();
        let expected = "COMMAND param1 param2";

        assert_eq!(&actual, expected);
    }

    #[test]
    fn with_trailing() {
        let message = Message {
            prefix: None,
            command: "COMMAND".to_string(),
            parameters: vec![],
            trailing: Some("trailing".to_string()),
        };

        let actual = message.to_string();
        let expected = "COMMAND :trailing";

        assert_eq!(&actual, expected);
    }

    #[test]
    fn full_message() {
        let message = Message {
            prefix: Some("prefix".to_string()),
            command: "COMMAND".to_string(),
            parameters: vec!["param1".to_string(), "param2".to_string()],
            trailing: Some("trailing".to_string()),
        };

        let actual = message.to_string();
        let expected = ":prefix COMMAND param1 param2 :trailing";

        assert_eq!(&actual, expected);
    }
}

#[cfg(test)]
mod tests_parsing {
    use super::*;

    // const FULL_MESSAGE: &str = ":prefix COMMAND param1 param2 :trailing with spaces";

    #[test]
    fn only_command() {
        let message = Message::new("COMMAND").unwrap();

        assert_eq!(None, message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(Vec::<String>::new(), message.parameters);
        assert_eq!(None, message.trailing);
    }

    #[test]
    fn w_prefix() {
        let message = Message::new(":prefix COMMAND").unwrap();

        assert_eq!(Some("prefix".to_string()), message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(Vec::<String>::new(), message.parameters);
        assert_eq!(None, message.trailing);
    }

    #[test]
    fn w_one_parameter() {
        let message = Message::new("COMMAND param1").unwrap();

        assert_eq!(None, message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(vec!["param1".to_string()], message.parameters);
        assert_eq!(None, message.trailing);
    }

    #[test]
    fn w_two_parameters() {
        let message = Message::new("COMMAND param1 param2").unwrap();

        assert_eq!(None, message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(
            vec!["param1".to_string(), "param2".to_string()],
            message.parameters
        );
        assert_eq!(None, message.trailing);
    }

    #[test]
    fn w_trailing() {
        let message = Message::new("COMMAND :trailing").unwrap();

        assert_eq!(None, message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(Vec::<String>::new(), message.parameters);
        assert_eq!(Some("trailing".to_string()), message.trailing);
    }

    #[test]
    fn w_trailing_w_spaces() {
        let message = Message::new("COMMAND :trailing with spaces").unwrap();

        assert_eq!(None, message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(Vec::<String>::new(), message.parameters);
        assert_eq!(Some("trailing with spaces".to_string()), message.trailing);
    }

    #[test]
    fn full_mesage() {
        let message = Message::new(":prefix COMMAND param1 param2 :trailing with spaces").unwrap();

        assert_eq!(Some("prefix".to_string()), message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(
            vec!["param1".to_string(), "param2".to_string()],
            message.parameters
        );
        assert_eq!(Some("trailing with spaces".to_string()), message.trailing);
    }
}
