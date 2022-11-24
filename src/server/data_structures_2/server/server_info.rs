pub struct ServerInfo {
    pub servername: String,
    pub serverinfo: String,
}

impl ServerInfo {
    pub fn new(servername: &str, serverinfo: &str) -> Self {
        Self {
            servername: servername.to_string(),
            serverinfo: serverinfo.to_string(),
        }
    }
}
