fn _is_numeric(a_value: &str) -> bool {
    a_value.chars().all(char::is_numeric)
}

fn _is_positive_numeric(a_value: &str) -> bool {
    _is_numeric(a_value) && a_value.parse::<isize>().unwrap() >= 0
}
