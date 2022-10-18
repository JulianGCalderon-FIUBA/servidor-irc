use std::net::TcpStream;

#[derive(Debug)]
pub struct ClientInfo {
    pub stream: TcpStream,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub servername: Option<String>,
    pub realname: Option<String>,
}

impl ClientInfo {
    pub fn with_stream(stream: TcpStream) -> Self {
        Self {
            stream,
            password: None,
            nickname: None,
            username: None,
            hostname: None,
            servername: None,
            realname: None,
        }
    }
}
