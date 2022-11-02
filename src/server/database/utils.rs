use super::ClientInfo;

pub fn index_of(element: String, array: &[String]) -> usize {
    array.iter().position(|client| *client == element).unwrap()
}

pub fn client_matches_nickmask(client: &ClientInfo, mask: &str) -> bool {
    matches(&client.nickname, mask)
}

pub fn client_matches_mask(client: &ClientInfo, query: &str) -> bool {
    if matches(&client.nickname, query) {
        return true;
    }
    if matches(&client.realname, query) {
        return true;
    }
    if matches(&client.username, query) {
        return true;
    }
    if matches(&client.hostname, query) {
        return true;
    }
    if matches(&client.servername, query) {
        return true;
    }

    false
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
