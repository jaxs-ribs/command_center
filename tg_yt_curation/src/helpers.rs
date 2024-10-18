use regex::Regex;
use sha2::{Sha256, Digest};
use crate::structs::{YoutubeEmbedParams, SetPostRequest, PostEntry};

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
    if let Some(start_time) = &params.start_time {
        if start_time != "0" {
            url.push_str(&format!("&start={}", start_time));
        }
    }
    
    // Add end time if it's present
    if let Some(end_time) = &params.end_time {
        url.push_str(&format!("&end={}", end_time));
    }
    url
}

pub fn is_curation_message(text: &str) -> bool {
    text.contains("youtu.be/") || text.contains("youtube.com/")
}

// video_id+<optional>start_time+<optional>end_time
pub fn hash_youtube_curation(youtube_embed_params: &YoutubeEmbedParams) -> String {
    let mut hasher = Sha256::new();
    let hash_input = format!("{}+{}+{}", 
        youtube_embed_params.video_id,
        youtube_embed_params.start_time.as_deref().unwrap_or(""),
        youtube_embed_params.end_time.as_deref().unwrap_or("")
    );
    hasher.update(hash_input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn create_post_entry(youtube_embed_params: &YoutubeEmbedParams) -> PostEntry {
    PostEntry {
        post_id: youtube_embed_params.video_id.clone(),
        post_json: format!("{:?}", youtube_embed_params),
    }
}

pub fn create_set_post_request(combined_uuid: &str, node: &str, post_entry: &PostEntry) -> SetPostRequest {
    SetPostRequest {
        stream_name: format!("{}-youtube", node),
        site: "youtubeVideo".to_string(),
        posts: vec![post_entry.clone()],
        combined_uuid: combined_uuid.to_string(),
    }
}