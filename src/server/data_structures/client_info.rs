#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
/// ClientInfo contains public Client information.
pub struct ClientInfo {
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
    pub hopcount: usize,
    pub operator: bool,
}
