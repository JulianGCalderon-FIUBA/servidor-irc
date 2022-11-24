use crate::server::consts::modes::*;
use crate::server::{connection::Connection, connection_handler::client_handler::ClientHandler};

use crate::server::data_structures_2::*;

impl<C: Connection> ClientHandler<C> {
    pub(super) fn channels_to_list(&mut self, channels: Option<&String>) -> Vec<String> {
        if channels.is_none() {
            return self.database.get_all_channels();
        }

        collect_list(channels)
    }

    pub(super) fn clients_for_default_who(&self) -> Vec<ClientInfo> {
        self.database
            .get_all_clients()
            .into_iter()
            .filter(|client_info| self.shares_channel_with(client_info))
            .collect()
    }

    pub(super) fn append_channel_role(&mut self, channels: &mut Vec<String>, nickname: &str) {
        for channel in channels {
            if self.database.is_channel_operator(channel, nickname) {
                channel.insert(0, OPERATOR_SYMBOL);
            } else if self
                .database
                .channel_has_mode(channel, &ChannelFlag::Moderated)
                && self.database.is_channel_speaker(channel, nickname)
            {
                channel.insert(0, SPEAKER_SYMBOL);
            }
        }
    }
}

pub fn parse_modes(modes: Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut add_modes: Vec<char> = vec![];
    let mut remove_modes: Vec<char> = vec![];
    let mut add: bool = false;
    for char in modes {
        match char {
            ADD_MODE => add = true,
            REMOVE_MODE => add = false,
            char => {
                if add {
                    add_modes.push(char);
                } else {
                    remove_modes.push(char);
                }
            }
        }
    }
    (add_modes, remove_modes)
}

pub fn collect_list(parameters: Option<&String>) -> Vec<String> {
    match parameters {
        Some(parameters) => parameters.split(',').map(|s| s.to_string()).collect(),
        None => vec![],
    }
}
