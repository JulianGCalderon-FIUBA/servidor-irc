use super::*;

/// Represents a Client that is connected to the Server.
pub struct ExternalClient {
    nicknames: Vec<String>,
    username: String,
    hostname: String,
    servername: String,
    realname: String,
    hopcount: usize,
    operator: bool,
    _away_message: Option<String>,
    _online: bool,
    _immediate_server: String,
}

impl ExternalClient {
    pub fn new(
        nickname: String,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
        hopcount: usize,
        _immediate_server: String,
    ) -> Self {
        Self {
            nicknames: vec![nickname],
            username,
            hostname,
            servername,
            realname,
            hopcount,
            operator: false,
            _away_message: None,
            _online: true,
            _immediate_server,
        }
    }

    pub fn get_info(&self) -> ClientInfo {
        ClientInfo {
            nickname: self.nickname(),
            username: self.username.clone(),
            hostname: self.hostname.clone(),
            servername: self.servername.clone(),
            realname: self.realname.clone(),
            operator: self.operator,
            hopcount: self.hopcount,
        }
    }

    /// Returns current nickname.
    pub fn nickname(&self) -> String {
        self.nicknames.last().unwrap().to_string()
    }

    /// Returns current nickname.
    pub fn _immediate_server(&self) -> String {
        self._immediate_server.clone()
    }
}
