use super::*;
use crate::server::consts::channel_flag::ChannelFlag;
use crate::server::testing::{dummy_client, dummy_database, MockTcpStream};

mod away_tests;
mod ctcp_tests;
mod invite_tests;
mod join_tests;
mod kick_tests;
mod list_tests;
mod mode_tests;
mod names_tests;
mod nick_tests;
mod notice_tests;
mod oper_tests;
mod part_tests;
mod privmsg_tests;
mod quit_tests;
mod squit_tests;
mod topic_tests;
mod who_tests;
mod whois_tests;

fn dummy_client_handler() -> ClientHandler<MockTcpStream> {
    let database = dummy_database();
    let nickname = "nickname".to_string();
    let online = Arc::new(AtomicBool::new(true));

    let client = dummy_client(&nickname);
    let connection = client.stream().unwrap().try_clone().unwrap();

    database.add_local_client(client);

    ClientHandler::from_connection(connection, nickname, database, online).unwrap()
}
