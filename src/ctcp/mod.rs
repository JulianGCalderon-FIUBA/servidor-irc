use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    macros::some_or_return,
    message::{Message, CRLF},
    server::consts::commands::PRIVMSG_COMMAND,
};

use self::dcc_connection::DccConnection;

pub mod dcc_connection;
pub mod dcc_message;

pub const CONTROL_CHARACTER: char = 1 as char;

pub fn is_ctcp_message(message: &Message) -> bool {
    let command = message.get_command();
    let trailing: Vec<char> = message.get_trailing().as_ref().unwrap().chars().collect();

    if command != PRIVMSG_COMMAND {
        return false;
    }

    let first = some_or_return!(trailing.first(), false);
    let last = some_or_return!(trailing.last(), false);

    first == &CONTROL_CHARACTER && last == &CONTROL_CHARACTER
}

pub fn get_ctcp_message(message: &Message) -> Option<String> {
    if !is_ctcp_message(message) {
        return None;
    }

    let mut content = message.get_trailing().to_owned().unwrap();

    content.remove(0);
    content.pop();

    Some(content)
}

pub fn issue_chat_command(server: &mut TcpStream, client: &str) -> io::Result<TcpListener> {
    let listener = TcpListener::bind("0.0.0.0:0")?;

    let address = listener.local_addr()?;

    let ip = address.ip();
    let port = address.port();

    write!(server, "CTCP {client} :DCC CHAT {ip} {port}")?;
    server.write_all(CRLF)?;

    Ok(listener)
}

pub fn receive_chat_accept(listener: TcpListener) -> io::Result<DccConnection> {
    let stream = listener.accept()?.0;

    DccConnection::new(stream)
}

pub fn receive_chat_decline(listener: TcpListener) {
    drop(listener)
}

pub fn accept_chat_command(
    server: &mut TcpStream,
    client: &str,
    address: &str,
) -> io::Result<DccConnection> {
    write!(server, "CTCP {client} :DCC CHAT accept")?;
    server.write_all(CRLF)?;

    DccConnection::connect(address)
}
pub fn decline_chat_command(server: &mut TcpStream, client: &str) -> io::Result<()> {
    write!(server, "CTCP {client} :DCC CHAT decline")?;
    server.write_all(CRLF)
}
