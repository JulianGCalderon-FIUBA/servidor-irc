use super::modes::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Possible flags a user may have
pub enum UserFlag {
    Invisible,
    Operator,
    ReceiveServerNotices,
    ReceivesWallops,
    InvalidFlag,
}

impl UserFlag {
    pub fn to_char(&self) -> char {
        match self {
            Self::Invisible => INVISIBLE,
            Self::Operator => OPERATOR,
            Self::ReceiveServerNotices => RECEIVES_SERVER_NOTICES,
            Self::ReceivesWallops => RECEIVES_WALLOPS,
            Self::InvalidFlag => panic!("Flag is invalid"),
        }
    }
    pub fn from_char(character: char) -> Self {
        match character {
            INVISIBLE => Self::Invisible,
            OPERATOR => Self::Operator,
            RECEIVES_SERVER_NOTICES => Self::ReceiveServerNotices,
            RECEIVES_WALLOPS => Self::ReceivesWallops,
            _ => Self::InvalidFlag,
        }
    }
}
