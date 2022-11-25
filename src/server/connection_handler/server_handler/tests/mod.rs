use crate::server::testing::{dummy_database, dummy_server, MockTcpStream};

mod invite_tests;
mod join_tests;
mod nick_tests;
mod part_tests;
mod privmsg_tests;
mod user_tests;

mod away_tests;

use super::*;

fn dummy_server_handler() -> ServerHandler<MockTcpStream> {
    let server = dummy_server("servername1");

    let database = dummy_database();
    let online = Arc::new(AtomicBool::new(true));
    let stream = server.get_stream().unwrap();

    database.add_immediate_server(server);

    ServerHandler::from_connection(stream, "servername1".to_string(), database, online).unwrap()
}
