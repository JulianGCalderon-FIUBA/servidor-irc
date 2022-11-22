mod mock_stream;

use super::{
    data_structures::*,
    database::{Database, DatabaseHandle},
};
pub use mock_stream::MockTcpStream;

/// Creates dummy client used for tests.
pub fn dummy_client(nickname: &str) -> Client<MockTcpStream> {
    let builder = ClientBuilder::new()
        .nickname(nickname.to_string())
        .username("username".to_string())
        .hostname("127.0.0.1".to_string())
        .servername("servername".to_string())
        .realname("realname".to_string())
        .stream(MockTcpStream::new());

    builder.build().unwrap()
}

pub fn dummy_database() -> DatabaseHandle<MockTcpStream> {
    let (handle, _) = Database::start("servername".to_string(), "serverinfo".to_string());
    handle
}
