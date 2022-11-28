use crate::{
    macros::{ok_or_return, some_or_return},
    server::consts::modes::{
        ChannelFlag, SET_BANMASK, SET_KEY, SET_OPERATOR, SET_SPEAKER, SET_USER_LIMIT,
    },
};

pub enum ModeRequest {
    AddBanmask(String),
    GetBanmasks,
    AddOperator(String),
    AddSpeaker(String),
    RemoveBanmask(String),
    RemoveOperator(String),
    RemoveSpeaker(String),
    SetFlag(ChannelFlag),
    SetKey(String),
    SetLimit(usize),
    UnsetLimit(),
    UnsetKey(),
    UnsetFlag(ChannelFlag),
    UnknownMode(char),
    NeedArgument(char),
    InvalidArgument(char, String),
}

impl ModeRequest {
    pub fn from(character: char, add: bool, arguments: &mut Vec<String>) -> Self {
        match add {
            true => match character {
                SET_USER_LIMIT => Self::build_set_limit_variant(arguments),
                SET_BANMASK => Self::build_add_banmask_variant(arguments),
                SET_SPEAKER => Self::build_add_speaker_variant(arguments),
                SET_KEY => Self::build_set_key_variant(arguments),
                SET_OPERATOR => Self::build_add_operator_variant(arguments),
                ch => Self::build_set_flag_variant(ch),
            },
            false => match character {
                SET_USER_LIMIT => Self::UnsetLimit(),
                SET_BANMASK => Self::build_remove_banmask_variant(arguments),
                SET_SPEAKER => Self::build_remove_speaker_variant(arguments),
                SET_KEY => Self::UnsetKey(),
                SET_OPERATOR => Self::build_remove_operator_variant(arguments),
                ch => Self::build_unset_flag_variant(ch),
            },
        }
    }

    fn build_set_limit_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let limit = some_or_return!(arguments.pop(), Self::NeedArgument(SET_USER_LIMIT));
        let limit = ok_or_return!(
            limit.parse::<usize>(),
            Self::InvalidArgument(SET_USER_LIMIT, limit)
        );
        Self::SetLimit(limit)
    }

    fn build_add_banmask_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let banmask = some_or_return!(arguments.pop(), Self::GetBanmasks);
        Self::AddBanmask(banmask)
    }

    fn build_remove_banmask_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let banmask = some_or_return!(arguments.pop(), Self::NeedArgument(SET_BANMASK));
        Self::RemoveBanmask(banmask)
    }

    fn build_add_speaker_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let speaker = some_or_return!(arguments.pop(), Self::NeedArgument(SET_SPEAKER));
        Self::AddSpeaker(speaker)
    }

    fn build_remove_speaker_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let speaker = some_or_return!(arguments.pop(), Self::NeedArgument(SET_SPEAKER));
        Self::RemoveSpeaker(speaker)
    }

    fn build_add_operator_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let operator = some_or_return!(arguments.pop(), Self::NeedArgument(SET_OPERATOR));
        Self::AddOperator(operator)
    }

    fn build_remove_operator_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let operator = some_or_return!(arguments.pop(), Self::NeedArgument(SET_OPERATOR));
        Self::RemoveOperator(operator)
    }

    fn build_set_key_variant(arguments: &mut Vec<String>) -> ModeRequest {
        let key = some_or_return!(arguments.pop(), Self::NeedArgument(SET_OPERATOR));
        Self::SetKey(key)
    }

    fn build_unset_flag_variant(character: char) -> ModeRequest {
        let flag = ChannelFlag::from_char(character);
        if let ChannelFlag::InvalidFlag = flag {
            return Self::UnknownMode(character);
        }

        Self::UnsetFlag(flag)
    }

    fn build_set_flag_variant(character: char) -> ModeRequest {
        let flag = ChannelFlag::from_char(character);
        if let ChannelFlag::InvalidFlag = flag {
            return Self::UnknownMode(character);
        }

        Self::SetFlag(flag)
    }
}
