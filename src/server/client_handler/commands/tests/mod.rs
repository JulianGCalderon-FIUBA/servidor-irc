mod invite;
mod join;
mod list;
mod names;
mod nick;
mod oper;
mod part;
mod pass;
mod privmsg;
mod user;

use std::sync::Arc;

use crate::server::database::Database;
use crate::server::testing_utils::dummy_client;
use crate::server::testing_utils::mock_stream::MockTcpStream;

use super::*;

fn dummy_client_handler() -> ClientHandler<MockTcpStream> {
    let database = Database::new();
    let handler_stream = MockTcpStream::new();
    let database_stream = handler_stream.clone();

    ClientHandler::new(Arc::new(database), handler_stream, database_stream).unwrap()
}

fn register_client(handler: &mut ClientHandler<MockTcpStream>, nick: &str) {
    let parameters = vec![nick.to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
    let trailing = Some("real".to_string());
    handler.user_command(parameters, trailing).unwrap();

    handler.stream_client_handler.clear()
}
