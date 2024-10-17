use regex::Regex;
use crate::structs::YoutubeEmbedParams;

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

pub fn extract_youtube_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?x)
        (?:https?://)?
        (?:www\.|m\.|.+\.)?
        (?:youtu\.be/|youtube\.com/
            (?:embed/|v/|shorts/|feeds/api/videos/|watch\?v=|watch\?.+&v=))
        ([\w-]{11})
    ").unwrap();

    re.captures(url)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

pub fn create_youtube_embed_src(params: &YoutubeEmbedParams) -> String {
    let base_url = "https://www.youtube.com/embed/";
    let mut url = format!("{}{}", base_url, params.video_id);
    
    // Start building query parameters
    url.push_str("?cc_load_policy=1&color=white");
    
    // Add start time if it's not empty or "0"
    if !params.start_time.is_empty() && params.start_time != "0" {
        url.push_str(&format!("&start={}", params.start_time));
    }
    
    // Add end time if it's not empty
    if !params.end_time.is_empty() {
        url.push_str(&format!("&end={}", params.end_time));
    }
    
    url
}

pub fn is_curation_message(text: &str) -> bool {
    text.contains("youtu.be/") || text.contains("youtube.com/")
}
