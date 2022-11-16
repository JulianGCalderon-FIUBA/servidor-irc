use crate::server::{database::Database, testing_utils::{MockTcpStream, dummy_client}};

use super::*;

mod away_tests;
mod invite_tests;
mod join_tests;
mod kick_tests;
mod list_tests;
mod mode_tests;
mod names_tests;
// mod nick_tests;
mod notice_tests;
mod oper_tests;
mod part_tests;
mod privmsg_tests;
// mod topic_tests;
// mod who_tests;
// mod whois_tests;

fn dummy_client_handler() -> ClientHandler<MockTcpStream> {
    let connection = MockTcpStream::new();
    let database = Database::start().0;
    let nickname = "nickname".to_string();
    let servername = "servername".to_string();
    let online = Arc::new(AtomicBool::new(true));

    let client = dummy_client(&nickname);

    database.add_client(client);

    ClientHandler::from_connection(connection, servername, nickname, database, online).unwrap()
}
