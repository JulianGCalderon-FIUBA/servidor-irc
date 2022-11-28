mod channel_mode_request;
mod user_mode_request;

pub use channel_mode_request::ChannelModeRequest;
pub use user_mode_request::UserModeRequest;

use crate::server::consts::modes::{ADD_MODE, REMOVE_MODE};

pub fn parse_channel_mode_string(
    mode_string: String,
    mut mode_arguments: Vec<String>,
) -> Vec<ChannelModeRequest> {
    let mut add: bool = Default::default();

    let mut requests = Vec::new();
    for char in mode_string.chars() {
        match char {
            ADD_MODE => add = true,
            REMOVE_MODE => add = false,
            char => requests.push(ChannelModeRequest::from(char, add, &mut mode_arguments)),
        }
    }
    requests
}

pub fn parse_user_mode_string(mode_string: String) -> Vec<UserModeRequest> {
    let mut add: bool = Default::default();

    let mut requests = Vec::new();
    for char in mode_string.chars() {
        match char {
            ADD_MODE => add = true,
            REMOVE_MODE => add = false,
            char => requests.push(UserModeRequest::from(char, add)),
        }
    }
    requests
}
