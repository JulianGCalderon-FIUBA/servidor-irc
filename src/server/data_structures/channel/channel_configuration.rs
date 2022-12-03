use crate::server::consts::channel_flag::ChannelFlag;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelConfiguration {
    pub operators: Vec<String>,
    pub banmasks: Vec<String>,
    pub speakers: Vec<String>,
    pub key: Option<String>,
    pub topic: Option<String>,
    pub user_limit: Option<usize>,
    pub flags: Vec<ChannelFlag>,
}

impl ChannelConfiguration {
    pub fn new() -> Self {
        Self {
            operators: vec![],
            banmasks: vec![],
            speakers: vec![],
            key: None,
            user_limit: None,
            topic: None,
            flags: vec![],
        }
    }
}

impl Default for ChannelConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
