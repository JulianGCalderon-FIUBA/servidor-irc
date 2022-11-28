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
    let servername = servername.to_string();
    let serverinfo = "serverinfo".to_string();
    ImmediateServer::new(stream, servername, serverinfo, 1)
}

pub fn dummy_database() -> DatabaseHandle<MockTcpStream> {
    let servername = "servername".to_string();
    let serverinfo = "serverinfo".to_string();
    let (handle, _) = Database::start(servername, serverinfo);
    handle
}

pub fn dummy_external_client(nickname: &str, servername: &str) -> ExternalClient {
    ClientBuilder::<MockTcpStream>::new()
        .nickname(nickname)
        .username("username")
        .hostname("127.0.0.1")
        .servername(servername)
        .realname("realname")
        .immediate(servername)
        .build_external_client()
        .unwrap()
}

pub fn dummy_distant_server(servername: &str) -> ServerInfo {
    ServerInfo::new(servername.to_string(), "serverinfo".to_string(), 2)
}

// pub fn dummy_distant_client(
//     nickname: &str,
//     servername: &str,
//     immediate_server: &str,
// ) -> ExternalClient {
//     ClientBuilder::<MockTcpStream>::new()
//         .nickname(nickname)
//         .username("username")
//         .hostname("127.0.0.1")
//         .servername(servername)
//         .realname("realname")
//         .immediate(immediate_server)
//         .build_external_client()
//         .unwrap()
// }
