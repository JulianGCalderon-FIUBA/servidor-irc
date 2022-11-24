#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
/// ClientInfo contains public Client information.
pub struct ClientInfo {
    pub nicknames: Vec<String>,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
    pub hopcount: usize,
    pub operator: bool,
}

impl ClientInfo {
    pub fn new(
        nickname: &str,
        username: &str,
        hostname: &str,
        servername: &str,
        realname: &str,
        hopcount: usize,
    ) -> Self {
        Self {
            nicknames: vec![nickname.to_string()],
            username: username.to_string(),
            hostname: hostname.to_string(),
            servername: servername.to_string(),
            realname: realname.to_string(),
            hopcount,
            operator: false,
        }
    }
}
