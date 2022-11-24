#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
/// ClientInfo contains public Client information.
pub struct ClientInfo {
    pub nicknames: Vec<String>,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
    pub hopcount: usize,
    pub operator: bool,
    pub away: Option<String>,
}

impl ClientInfo {
    pub fn new(
        nickname: &str,
        username: &str,
        hostname: &str,
        servername: &str,
        realname: &str,
        hopcount: usize,
    ) -> Self {
        Self {
            nicknames: vec![nickname.to_string()],
            username: username.to_string(),
            hostname: hostname.to_string(),
            servername: servername.to_string(),
            realname: realname.to_string(),
            hopcount,
            operator: false,
            away: None,
        }
    }

    pub fn nickname(&self) -> String {
        self.nicknames.last().unwrap().clone()
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
}

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
