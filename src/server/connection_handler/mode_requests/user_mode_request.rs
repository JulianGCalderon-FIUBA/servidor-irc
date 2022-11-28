use crate::server::consts::modes::UserFlag;

pub enum UserModeRequest {
    SetFlag(UserFlag),
    UnsetFlag(UserFlag),
    UnknownRequest(char),
}

impl UserModeRequest {
    pub fn from(character: char, add: bool) -> Self {
        match add {
            true => Self::build_set_flag_variant(character),
            false => Self::build_unset_flag_variant(character),
        }
    }

    pub(crate) fn build_set_flag_variant(character: char) -> UserModeRequest {
        let flag = UserFlag::from_char(character);
        if let UserFlag::InvalidFlag = flag {
            return Self::UnknownRequest(character);
        }

        Self::SetFlag(flag)
    }

    pub(crate) fn build_unset_flag_variant(character: char) -> UserModeRequest {
        let flag = UserFlag::from_char(character);
        if let UserFlag::InvalidFlag = flag {
            return Self::UnknownRequest(character);
        }

        Self::UnsetFlag(flag)
    }
}

impl std::fmt::Display for UserModeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserModeRequest::SetFlag(flag) => write!(f, "+{}", flag.to_char()),
            UserModeRequest::UnsetFlag(flag) => write!(f, "-{}", flag.to_char()),
            UserModeRequest::UnknownRequest(_) => Ok(()),
        }
    }
}
