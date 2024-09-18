
pub fn parse_register_command(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() == 2 && parts[0] == "/register" {
        Some(parts[1].to_string())
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

pub fn parse_twitter_link(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.starts_with("https://twitter.com/") || trimmed.starts_with("https://x.com/") {
        trimmed.split('/').last().map(String::from)
    } else {
        None
    }
}