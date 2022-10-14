mod creation_error;
mod parsing;
mod parsing_error;

pub use creation_error::CreationError;
pub use parsing_error::ParsingError;

use std::io::{self, BufRead, BufReader, ErrorKind};
use std::io::{Read, Write};
pub struct Message {
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
    trailing: Option<String>,
}

const CRLF: &[u8] = b"\r\n";
const PREFIX_CHARACTER: u8 = b':';
const MAX_LENGTH: usize = 510;
const INVALID_CHARACTERS: [char; 3] = ['\r', '\n', '\0'];

impl Message {
    pub fn new(content: &str) -> Result<Self, ParsingError> {
        let (prefix, command, parameters, trailing) = parsing::parse(content)?;

        Ok(Self {
            prefix,
            command,
            parameters,
            trailing,
        })
    }

    pub fn send_to(&self, stream: &mut dyn Write) -> io::Result<()> {
        let string = self.to_string();
        let bytes = string.as_bytes();

        stream.write_all(bytes)?;
        stream.write_all(CRLF)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, CreationError> {
        let mut reader = BufReader::new(stream);

        let mut content = String::new();

        let size = reader.read_line(&mut content)?;
        if size == 0 {
            return Err(CreationError::IoError(unexpected_eof_error()));
        }

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return Err(CreationError::ParsingError(ParsingError::NoTrailingCRLF));
        }

        let message = Self::new(&content)?;

        Ok(message)
    }

    pub fn unpack(self) -> (Option<String>, String, Vec<String>, Option<String>) {
        (self.prefix, self.command, self.parameters, self.trailing)
    }
}

fn unexpected_eof_error() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, "Encountered EOF")
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

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("prefix", &self.prefix)
            .field("command", &self.command)
            .field("parameters", &self.parameters)
            .field("trailing", &self.trailing)
            .finish()
    }
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
    fn with_trailing_with_spaces() {
        let message = Message {
            prefix: None,
            command: "COMMAND".to_string(),
            parameters: vec![],
            trailing: Some("trailing with spaces".to_string()),
        };

        let actual = message.to_string();
        let expected = "COMMAND :trailing with spaces";

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
    fn full_message() {
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
