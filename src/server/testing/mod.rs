mod mock_stream;

use super::database::{Client, ClientBuilder};
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