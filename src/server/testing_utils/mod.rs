pub mod mock_stream;

use super::database::{Client, ClientBuilder};
use mock_stream::MockTcpStream;

pub fn dummy_client(nickname: &str) -> Client<MockTcpStream> {
    let builder = ClientBuilder::new()
        .nickname(nickname.to_string())
        .username("username".to_string())
        .hostname("hostname".to_string())
        .servername("servername".to_string())
        .realname("realname".to_string())
        .stream(MockTcpStream::new());

    builder.build().unwrap()
}
