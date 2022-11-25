pub struct ServerInfo {
    pub servername: String,
    pub serverinfo: String,
    pub hopcount: usize,
}

impl ServerInfo {
    pub fn new(servername: &str, serverinfo: &str, hopcount: usize) -> Self {
        Self {
            servername: servername.to_string(),
            serverinfo: serverinfo.to_string(),
            hopcount,
        }
    }
}
