use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::io::{Read, Write};

pub struct Message {
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
    trailing: Option<String>,
}

type MessageParse = (Option<String>, String, Vec<String>, Option<String>);

const CRLF: &[u8] = b"\r\n";

impl Message {
    pub fn new(content: String) -> io::Result<Self> {
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
            return Err(eof_error());
        }

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return Err(no_trailing_crlf_in_message_error());
        }

        Self::new(content)
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

fn parse(content: String) -> io::Result<MessageParse> {
    if content.is_empty() {
        return Err(empty_message_error());
    }

    Ok((None, content, Vec::new(), None))
}

fn eof_error() -> Error {
    Error::new(ErrorKind::UnexpectedEof, "encountered EOF")
}

fn empty_message_error() -> Error {
    Error::new(ErrorKind::InvalidInput, "message should not be empty")
}

fn no_trailing_crlf_in_message_error() -> Error {
    Error::new(ErrorKind::InvalidInput, "message should end in CRLF")
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

    // const W_ONE_PARAMETER: &str = "COMMAND param1";
    // const W_TWO_PARAMETER: &str = "COMMAND param1 param2";
    // const W_TRAILING: &str = "COMMAND :trailing";
    // const W_TRAILING_W_SPACES: &str = "COMMAND :trailing with spaces";
    // const FULL_MESSAGE: &str = ":prefix COMMAND param1 param2 :trailing with spaces";

    #[test]
    fn only_command() {
        let message = Message::new("COMMAND".to_string()).unwrap();

        assert_eq!(None, message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(Vec::<String>::new(), message.parameters);
        assert_eq!(None, message.trailing);
    }

    #[test]
    fn w_prefix() {
        let message = Message::new(":prefix COMMAND".to_string()).unwrap();

        assert_eq!(Some("prefix".to_string()), message.prefix);
        assert_eq!("COMMAND", &message.command);
        assert_eq!(Vec::<String>::new(), message.parameters);
        assert_eq!(None, message.trailing);
    }
}
