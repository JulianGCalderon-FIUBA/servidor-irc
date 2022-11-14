mod away;
mod invite;
mod join;
mod kick;
mod list;
mod mode;
mod names;
mod nick;
mod notice;
mod oper;
mod part;
mod pass;
mod privmsg;
mod topic;
mod user;
mod who;
mod whois;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::server::database::Database;
use crate::server::testing_utils::dummy_client;
use crate::server::testing_utils::mock_stream::MockTcpStream;

use super::*;

fn dummy_client_handler() -> ClientHandler<MockTcpStream> {
    let database = Database::start();
    let stream = MockTcpStream::new();

    let online = Arc::new(AtomicBool::new(true));
    ClientHandler::from_stream(database, stream, "servername".to_string(), online).unwrap()
}

fn register_client(handler: &mut ClientHandler<MockTcpStream>, nickname: &str) {
    let parameters = vec![nickname.to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec![
        "username".to_string(),
        "hostname".to_string(),
        "servername".to_string(),
    ];
    let trailing = Some("realname".to_string());
    handler.user_command(parameters, trailing).unwrap();

    handler.stream.clear()
}
