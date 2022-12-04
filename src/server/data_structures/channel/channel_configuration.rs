use crate::server::consts::channel_flag::ChannelFlag;

#[derive(Clone, Debug, PartialEq, Eq)]
/// Contains a Channels's configuration.
pub struct ChannelConfiguration {
    /// channel operators. They have privileges to modify the channel.
    pub operators: Vec<String>,
    /// channel banmasks, if a user matches them they cannot join.
    pub banmasks: Vec<String>,
    /// when channel is in moderated mode, only speakers are allowed to send messages.
    pub speakers: Vec<String>,
    /// a channel may have a key.
    pub key: Option<String>,
    /// a channel may have a topic.
    pub topic: Option<String>,
    /// a channel may have a user limit.
    pub user_limit: Option<usize>,
    /// stores the channel's flags.
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
