use std::collections::HashMap;

use crate::server::consts::user_flag::UserFlag;

#[derive(PartialEq, Eq, Debug, Clone)]
/// ClientInfo contains public client information.
pub struct ClientInfo {
    /// stores history of client's nicknames
    pub nicknames: Vec<String>,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
    /// how many servers away is the client from the local server
    pub hopcount: usize,
    /// a client may have an away message
    pub away: Option<String>,
    /// stores the user's flags
    pub flags: HashMap<UserFlag, ()>,
}

impl ClientInfo {
    pub fn nickname(&self) -> String {
        self.nicknames
            .last()
            .expect("There should always be at least one nickname")
            .clone()
    }

    pub fn matches_banmask(&self, query: &str) -> bool {
        if matches(&self.nickname(), query) {
            return true;
        }

        if matches(&self.username, query) {
            return true;
        }
        if matches(&self.hostname, query) {
            return true;
        }

        false
    }

    pub fn is_operator(&self) -> bool {
        self.flags.contains_key(&UserFlag::Operator)
    }

    pub fn matches_mask(&self, query: &str) -> bool {
        if matches(&self.nickname(), query) {
            return true;
        }
        if matches(&self.username, query) {
            return true;
        }
        if matches(&self.hostname, query) {
            return true;
        }
        if matches(&self.realname, query) {
            return true;
        }
        if matches(&self.servername, query) {
            return true;
        }

        false
    }

    pub fn matches_nickmask(&self, query: &str) -> bool {
        matches(&self.nickname(), query)
    }

    pub fn update_nickname(&mut self, nickname: String) {
        self.nicknames.push(nickname)
    }

    pub fn add_flag(&mut self, flag: UserFlag) {
        self.flags.insert(flag, ());
    }

    pub fn remove_flag(&mut self, flag: UserFlag) {
        self.flags.remove(&flag);
    }
}
/// Iterates through base checking if pattern matches it in some way.
/// '?' matches any character
/// '*' matches any sequence of characters
pub fn matches(base: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return base.is_empty();
    }
    let base = base.as_bytes();
    let pattern = pattern.as_bytes();

    let mut base_index = 0;
    let mut pattern_index = 0;
    let mut glob_base_index = -1;
    let mut glob_pattern_index = -1;

    while base_index < base.len() {
        if pattern_index < pattern.len() {
            if base[base_index] == pattern[pattern_index] || pattern[pattern_index] == b'?' {
                base_index += 1;
                pattern_index += 1;
                continue;
            }
            if pattern[pattern_index] == b'*' {
                glob_base_index = base_index as isize;
                glob_pattern_index = pattern_index as isize;
                pattern_index += 1;
                continue;
            }
        }
        if glob_pattern_index != -1 {
            base_index = (glob_base_index + 1) as usize;
            pattern_index = (glob_pattern_index + 1) as usize;
            glob_base_index += 1;
            continue;
        }
        return false;
    }

    while pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
        pattern_index += 1;
    }

    pattern_index == pattern.len()
}
