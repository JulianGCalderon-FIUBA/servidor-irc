pub mod mock_stream;

use std::sync::{Arc, Mutex};

use super::database::{ClientInfo, ClientInfoBuilder};
use mock_stream::MockTcpStream;

pub fn dummy_client(nickname: &str) -> ClientInfo<MockTcpStream> {
    let mut builder = ClientInfoBuilder::new_with(
        nickname.to_string(),
        "username".to_string(),
        "hostname".to_string(),
        "servername".to_string(),
        "real name".to_string(),
    );

    builder.with_stream(Arc::new(Mutex::new(MockTcpStream::new())));

    builder.build()
}
