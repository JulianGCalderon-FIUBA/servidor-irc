mod mock_stream;

use super::{
    data_structures::*,
    database::{Database, DatabaseHandle},
};
pub use mock_stream::MockTcpStream;

/// Creates dummy client used for tests.
pub fn dummy_client(nickname: &str) -> LocalClient<MockTcpStream> {
    ClientBuilder::new()
        .nickname(nickname)
        .username("username")
        .hostname("127.0.0.1")
        .servername("servername")
        .realname("realname")
        .stream(MockTcpStream::new())
        .build_local_client()
        .unwrap()
}

pub fn dummy_server(servername: &str) -> ImmediateServer<MockTcpStream> {
    let stream = MockTcpStream::new();
    ImmediateServer::new(stream, servername, "serverinfo", 1)
}

pub fn dummy_database() -> DatabaseHandle<MockTcpStream> {
    let (handle, _) = Database::start("servername", "serverinfo");
    handle
}
