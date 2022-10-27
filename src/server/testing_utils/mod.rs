pub mod mock_stream;

use super::database::{ClientInfo, ClientInfoBuilder};
use mock_stream::MockTcpStream;

pub fn dummy_client(nickname: &str) -> ClientInfo<MockTcpStream> {
    let builder = ClientInfoBuilder::new()
        .nickname(nickname.to_string())
        .username("username".to_string())
        .hostname("hostname".to_string())
        .servername("servername".to_string())
        .realname("real name".to_string())
        .stream(MockTcpStream::new());

    builder.build().unwrap()
}
