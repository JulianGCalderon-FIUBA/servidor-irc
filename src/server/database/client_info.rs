use std::net::TcpStream;

pub struct ClientInfo {
    pub stream: Option<TcpStream>,
    pub password: Option<String>,
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
}

impl ClientInfo {
    pub fn new_with_stream(
        stream: TcpStream,
        password: Option<String>,
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
