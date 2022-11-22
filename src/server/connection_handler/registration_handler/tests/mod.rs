use crate::server::testing::{dummy_database, MockTcpStream};

use super::*;

mod nick_tests;
mod non_registration_tests;
mod pass_tests;
mod user_tests;

fn dummy_registration_handler() -> RegistrationHandler<MockTcpStream> {
    let stream = MockTcpStream::new();
    let database = dummy_database();
    let online = Arc::new(AtomicBool::new(true));
    RegistrationHandler::from_connection(stream, database, online).unwrap()
}
