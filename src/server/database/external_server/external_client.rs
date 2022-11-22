use crate::server::database::ClientInfo;

/// Represents a Client that is connected to the Server.
pub struct ExternalClient {
    nicknames: Vec<String>,
    username: String,
    hostname: String,
    servername: String,
    realname: String,
    hopcount: usize,
    operator: bool,
}

impl ExternalClient {
    pub fn new(
        nickname: String,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
        hopcount: usize,
    ) -> Self {
        Self {
            nicknames: vec![nickname],
            username,
            hostname,
            servername,
            realname,
            hopcount,
            operator: false,
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

    // /// Updates nickname.
    // pub fn update_nickname(&mut self, nickname: String) {
    //     self.nicknames.push(nickname);
    // }

    // /// Returns true if Client has or had received nickname.
    // pub fn had_nickname(&self, nickname: &str) -> bool {
    //     self.nicknames.contains(&nickname.to_string())
    // }
}
