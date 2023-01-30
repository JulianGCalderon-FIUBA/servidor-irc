use std::fmt::Display;

const CHAT_TYPE: &str = "CHAT";
const SEND_TYPE: &str = "SEND";

pub enum DccType {
    Chat,
    Send,
    Unknown,
}

impl Display for DccType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DccType::Chat => write!(f, "{CHAT_TYPE}"),
            DccType::Send => write!(f, "{SEND_TYPE}"),
            DccType::Unknown => panic!("Unknown type can't be displayed"),
        }
    }
}

impl From<&str> for DccType {
    fn from(value: &str) -> Self {
        match value {
            CHAT_TYPE => Self::Chat,
            SEND_TYPE => Self::Send,
            _ => Self::Unknown,
        }
    }
}
