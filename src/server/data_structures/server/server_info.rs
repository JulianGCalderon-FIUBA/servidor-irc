pub struct ServerInfo {
    pub servername: String,
    pub serverinfo: String,
    pub hopcount: usize,
}

impl ServerInfo {
    pub fn new(servername: String, serverinfo: String, hopcount: usize) -> Self {
        Self {
            servername,
            serverinfo,
            hopcount,
        }
    }
}
