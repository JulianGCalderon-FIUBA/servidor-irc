use std::net::TcpStream;

pub struct ClientInfo {
    pub stream: Option<TcpStream>,
    pub password: String,
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
}

impl ClientInfo {
    pub fn new(
        password: String,
        nickname: String,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
    ) -> Self {
        Self {
            stream: None,
            password,
            nickname,
            username,
            hostname,
            servername,
            realname,
        }
    }
    pub fn new_with_stream(
        stream: TcpStream,
        password: String,
        nickname: String,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
    ) -> Self {
        Self {
            stream: Some(stream),
            password,
            nickname,
            username,
            hostname,
            servername,
            realname,
        }
    }
}
