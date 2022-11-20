/// Represents a Client that is connected to the Server.
pub struct ExternalClient {
    _nicknames: Vec<String>,
    _username: String,
    _hostname: String,
    _servername: String,
    _realname: String,
    _hopcount: usize,
}

impl ExternalClient {
    pub fn _new(
        nickname: String,
        _username: String,
        _hostname: String,
        _servername: String,
        _realname: String,
        _hopcount: usize,
    ) -> Self {
        Self {
            _nicknames: vec![nickname],
            _username,
            _hostname,
            _servername,
            _realname,
            _hopcount,
        }
    }

    // /// Updates nickname.
    // pub fn update_nickname(&mut self, nickname: String) {
    //     self.nicknames.push(nickname);
    // }

    // /// Returns current nickname.
    // pub fn nickname(&self) -> String {
    //     self.nicknames.last().unwrap().to_string()
    // }

    // /// Returns true if Client has or had received nickname.
    // pub fn had_nickname(&self, nickname: &str) -> bool {
    //     self.nicknames.contains(&nickname.to_string())
    // }
}
