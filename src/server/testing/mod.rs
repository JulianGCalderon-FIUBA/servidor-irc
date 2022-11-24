mod mock_stream;

use super::{
    data_structures::*,
    database::{Database, DatabaseHandle},
};
pub use mock_stream::MockTcpStream;

/// Creates dummy client used for tests.
pub fn dummy_client(nickname: &str) -> LocalClient<MockTcpStream> {
    let info = ClientInfo::new(
        nickname,
        "username",
        "127.0.0.1",
        "servername",
        "realname",
        1,
    );
    LocalClient::new(MockTcpStream::new(), &None, info)
}

pub fn dummy_server(servername: &str) -> ImmediateServer<MockTcpStream> {
    let stream = MockTcpStream::new();
    ImmediateServer::new(stream, servername, "serverinfo", 1)
}

pub fn dummy_database() -> DatabaseHandle<MockTcpStream> {
    let (handle, _) = Database::start("servername", "serverinfo");
    handle
}
