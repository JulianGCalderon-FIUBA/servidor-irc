use crate::server::consts::modes::ChannelFlag;
#[derive(Clone)]
pub struct ChannelConfig {
    pub operators: Vec<String>,
    pub banmasks: Vec<String>,
    pub speakers: Vec<String>,
    pub key: Option<String>,
    pub user_limit: Option<usize>,
    pub flags: Vec<ChannelFlag>,
}

impl ChannelConfig {
    pub fn new() -> Self {
        Self {
            operators: vec![],
            banmasks: vec![],
            speakers: vec![],
            key: None,
            user_limit: None,
            flags: vec![],
        }
    }
}
