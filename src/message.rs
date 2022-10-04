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
            return Err(empty_message_error());
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
    Ok((None, content, Vec::new(), None))
}

fn empty_message_error() -> Error {
    Error::new(ErrorKind::InvalidInput, "message should not be empty")
}

fn no_trailing_crlf_in_message_error() -> Error {
    Error::new(ErrorKind::InvalidInput, "message should end in CRLF")
}

#[cfg(test)]
mod tests {

    use super::*;

    // Messages
    // const ONLY_COMMAND: &str = "COMMAND";
    // const W_PREFIX: &str = ":prefix COMMAND";
    // const W_ONE_PARAMETER: &str = "COMMAND param1";
    // const W_TWO_PARAMETER: &str = "COMMAND param1 param2";
    // const W_TRAILING: &str = "COMMAND :trailing";
    // const W_TRAILING_W_SPACES: &str = "COMMAND :trailing with spaces";
    // const FULL_MESSAGE: &str = ":prefix COMMAND param1 param2 :trailing with spaces";

    #[test]
    fn only_command_print() {
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
    fn with_prefix_print() {
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
    fn with_one_parameter_print() {
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
    fn with_two_parameters_print() {
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
    fn with_trailing_print() {
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
    fn full_message_print() {
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
