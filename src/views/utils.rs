use super::MESSAGE_MAX_LINE_CHARACTERS;

pub fn do_break_line(text: &str) -> String {
    text.chars()
        .collect::<Vec<char>>()
        .chunks(MESSAGE_MAX_LINE_CHARACTERS)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(
            "
",
        )
}
