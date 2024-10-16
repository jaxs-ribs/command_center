
pub fn parse_register_command(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() == 2 && parts[0] == "/register" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

pub fn parse_start_command(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() == 1 && parts[0] == "/start" {
        Some(parts[0].to_string())
    } else {
        None
    }
}

pub fn parse_six_digit_number(text: &str) -> Option<u64> {
    if text.len() == 6 && text.chars().all(|c| c.is_digit(10)) {
        text.parse().ok()
    } else {
        None
    }
}


pub fn is_curation_message(text: &str) -> bool {
    text.contains("youtu.be/") || text.contains("youtube.com/")
}
