use crate::server::consts::modes::UserFlag;

pub enum UserModeRequest {
    SetFlag(UserFlag),
    UnsetFlag(UserFlag),
    UnknownRequest(char),
}

impl UserModeRequest {
    pub fn from(character: char, add: bool) -> Self {
        println!("char {character}");

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
