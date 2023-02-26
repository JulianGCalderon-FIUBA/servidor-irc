mod dcc_parsing_error;

#[cfg(test)]
mod tests;

use std::{net::SocketAddr, str::FromStr};

use crate::macros::{ok_or_return, some_or_return};

use self::dcc_parsing_error::DccParsingError;

const CHAT_TYPE: &str = "CHAT";
const SEND_TYPE: &str = "SEND";
const RESUME_TYPE: &str = "RESUME";
const ACCEPT_TYPE: &str = "ACCEPT";
const CLOSE_TYPE: &str = "CLOSE";

const DCC: &str = "DCC";

const CHAT_CHAT_PROTOCOL: &str = "chat";
const CHAT_ACCEPT_PROTOCOL: &str = "accept";
const CHAT_DECLINE_PROTOCOL: &str = "decline";

const SEND_ACCEPT_PROTOCOL: &str = "accept";
const SEND_DECLINE_PROTOCOL: &str = "decline";

/*
   DCC CHAT accept
   DCC CHAT decline
*/

#[derive(Debug)]
pub enum DccMessage {
    Send {
        filename: String,
        address: SocketAddr,
        filesize: u64,
    },
    SendAccept,
    SendDecline,
    Chat {
        address: SocketAddr,
    },
    ChatAccept,
    ChatDecline,
    Close,
    Resume {
        filename: String,
        port: u16,
        position: u64,
    },
    Accept {
        filename: String,
        port: u16,
        position: u64,
    },
}

impl DccMessage {
    pub fn parse(message: String) -> Result<Self, DccParsingError> {
        let mut arguments: Vec<String> = message.split(' ').map(|s| s.to_string()).collect();
        arguments.reverse();

        let dcc = some_or_return!(arguments.pop(), Err(DccParsingError::EmptyMessage));

        if dcc != DCC {
            return Err(DccParsingError::NoDcc);
        }

        let command = some_or_return!(arguments.pop(), Err(DccParsingError::NoCommand));
        match &command[..] {
            CHAT_TYPE => parse_chat_command(arguments),
            SEND_TYPE => parse_send_command(arguments),
            RESUME_TYPE => parse_resume_command(arguments),
            ACCEPT_TYPE => parse_accept_command(arguments),
            CLOSE_TYPE => Self::close_variant(),
            _ => Err(DccParsingError::UnknownCommand),
        }
    }

    fn close_variant() -> Result<DccMessage, DccParsingError> {
        Ok(DccMessage::Close {})
    }
}

fn parse_chat_command(mut arguments: Vec<String>) -> Result<DccMessage, DccParsingError> {
    let protocol = some_or_return!(arguments.pop(), Err(DccParsingError::NoProtocol));

    match &protocol[..] {
        CHAT_ACCEPT_PROTOCOL => Ok(DccMessage::ChatAccept),
        CHAT_DECLINE_PROTOCOL => Ok(DccMessage::ChatDecline),
        CHAT_CHAT_PROTOCOL => parse_chat_chat_command(arguments),
        _ => Err(DccParsingError::InvalidProtocol),
    }
}

fn parse_chat_chat_command(mut arguments: Vec<String>) -> Result<DccMessage, DccParsingError> {
    let address = parse_address(&mut arguments)?;
    Ok(DccMessage::Chat { address })
}

fn parse_send_command(mut arguments: Vec<String>) -> Result<DccMessage, DccParsingError> {
    let protocol = some_or_return!(arguments.pop(), Err(DccParsingError::NoFilename));

    if arguments.is_empty() {
        if protocol == SEND_ACCEPT_PROTOCOL {
            return Ok(DccMessage::SendAccept);
        } else if protocol == SEND_DECLINE_PROTOCOL {
            return Ok(DccMessage::SendDecline);
        }
    }

    let filename = protocol;

    let address = parse_address(&mut arguments)?;

    let filesize = parse_filesize(&mut arguments)?;

    Ok(DccMessage::Send {
        filename,
        address,
        filesize,
    })
}

fn parse_filesize(arguments: &mut Vec<String>) -> Result<u64, DccParsingError> {
    let filesize = some_or_return!(arguments.pop(), Err(DccParsingError::NoFilesize));

    let filesize = ok_or_return!(
        filesize.parse::<u64>(),
        Err(DccParsingError::InvalidFilesize)
    );

    Ok(filesize)
}

fn parse_resume_command(mut arguments: Vec<String>) -> Result<DccMessage, DccParsingError> {
    let filename = some_or_return!(arguments.pop(), Err(DccParsingError::NoFilename));

    let port = some_or_return!(arguments.pop(), Err(DccParsingError::NoPort));
    let port = ok_or_return!(port.parse::<u16>(), Err(DccParsingError::InvalidPort));

    let position = parse_position(&mut arguments)?;

    Ok(DccMessage::Resume {
        filename,
        port,
        position,
    })
}

fn parse_position(arguments: &mut Vec<String>) -> Result<u64, DccParsingError> {
    let position = some_or_return!(arguments.pop(), Err(DccParsingError::NoPosition));
    let position = ok_or_return!(
        position.parse::<u64>(),
        Err(DccParsingError::InvalidPosition)
    );
    Ok(position)
}

fn parse_accept_command(arguments: Vec<String>) -> Result<DccMessage, DccParsingError> {
    if let DccMessage::Resume {
        filename,
        port,
        position,
    } = parse_resume_command(arguments)?
    {
        return Ok(DccMessage::Accept {
            filename,
            port,
            position,
        });
    }

    panic!("if condition should always be met");
}

fn parse_address(arguments: &mut Vec<String>) -> Result<SocketAddr, DccParsingError> {
    let ip = some_or_return!(arguments.pop(), Err(DccParsingError::NoIp));
    let port = some_or_return!(arguments.pop(), Err(DccParsingError::NoPort));

    let formatted_address = format!("{ip}:{port}");
    let socket_address = ok_or_return!(
        SocketAddr::from_str(&formatted_address),
        Err(DccParsingError::InvalidAddress)
    );

    Ok(socket_address)
}
