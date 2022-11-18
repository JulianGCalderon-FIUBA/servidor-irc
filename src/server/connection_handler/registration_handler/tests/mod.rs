use crate::server::{database::Database, testing::MockTcpStream};

use super::*;

mod nick_tests;
mod non_registration_tests;
mod pass_tests;
mod user_tests;

fn dummy_registration_handler() -> RegistrationHandler<MockTcpStream> {
    let stream = MockTcpStream::new();
    let servername = "servername".to_string();
    let database = Database::start().0;
    let online = Arc::new(AtomicBool::new(true));
    RegistrationHandler::from_connection(stream, servername, database, online).unwrap()
}
