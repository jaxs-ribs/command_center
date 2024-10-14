use kinode_process_lib::{
    get_blob,
    http::client::{HttpClientAction, OutgoingHttpRequest},
    println, LazyLoadBlob, Request,
};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::ImgServerRequest;
use crate::ImgServerResponse;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_API_KEY: &str = include_str!("../../OPENAI_API_KEY");
pub const IMG_SERVER_ADDRESS: (&str, &str, &str, &str) =
    ("recentered.os", "img_server", "img_server", "uncentered.os");

fn get_base_64_img_from_server(uri: String) -> anyhow::Result<String> {
    let response = Request::to(IMG_SERVER_ADDRESS)
        .body(ImgServerRequest::GetImage(uri))
        .send_and_await_response(10)??;
    match response.body().try_into()? {
        ImgServerResponse::GetImage(Ok(b64_img)) => Ok(b64_img),
        _ => Err(anyhow::anyhow!(
            "Failed to upload image: unexpected response"
        )),
    }
}

// Converts URIs to base64 images, otherwise returns the original URL
fn process_image_urls(img_urls: Vec<String>) -> Vec<String> {
    let mut processed_urls = Vec::new();

    for url in img_urls {
        let processed_url = if url.starts_with("http://") || url.starts_with("https://") {
            url
        } else {
            match get_base_64_img_from_server(url) {
                Ok(b64_img) => format!("data:image/jpeg;base64,{}", b64_img),
                Err(e) => {
                    println!("Failed to fetch image: {}", e);
                    continue;
                }
            }
        };
        processed_urls.push(processed_url);
    }

    processed_urls
}

fn create_request_body(images: Vec<String>, content: &str) -> Value {
    json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": "You are an expert in content analysis, semantic understanding, and information retrieval. Your task is to generate rich, nuanced descriptions that will be used for advanced search and clustering algorithms."
            },
            {
                "role": "user",
                "content": create_user_content(images, content)
            }
        ],
        "max_tokens": 1000
    })
}

fn create_user_content(images: Vec<String>, content: &str) -> Vec<Value> {
    let mut user_content = vec![json!({
        "type": "text",
        "text": "
        You will be given a post, and possibly some images. 

        1. If there are images, describe images in detail for embedding and semantic search.
        2. Extract subtext and hidden meanings:
            - Focus on elements the embedding model may miss
            - Avoid repeating obvious content from image description
            - Use bullet points with tags or short sentences
            - Explain jokes/hidden meanings briefly
            - Include non-obvious associated words and deep connections
            - Prioritize unique, search-relevant content

        Aim for comprehensive yet concise descriptions that facilitate effective searching and clustering.
        "
    })];

    let image_content = images.into_iter().map(|img| {
        json!({
            "type": "image_url",
            "image_url": {
                "url": img
            }
        })
    }).collect::<Vec<_>>();
    println!("----");
    println!("{:?}", image_content);
    println!("Text content: {:?}", content);
    println!("----");
    user_content.extend(image_content);

    user_content.push(json!({
        "type": "text",
        "text": format!("Accompanying text content: {}", content)
    }));

    user_content
}

fn send_request(request_body: Value) -> Result<String, String> {
    let headers = HashMap::from_iter(vec![
        ("Content-Type".to_string(), "application/json".to_string()),
        (
            "Authorization".to_string(),
            format!("Bearer {}", OPENAI_API_KEY),
        ),
    ]);

    let outgoing_request = OutgoingHttpRequest {
        method: "POST".to_string(),
        version: None,
        url: OPENAI_API_URL.to_string(),
        headers,
    };

    let body = serde_json::to_vec(&HttpClientAction::Http(outgoing_request))
        .map_err(|e| format!("Failed to serialize request: {}", e))?;

    let blob = LazyLoadBlob {
        mime: Some("application/json".to_string()),
        bytes: serde_json::to_vec(&request_body)
            .map_err(|e| format!("Failed to serialize request body: {}", e))?,
    };

    let _response = Request::new()
        .target(("our", "http_client", "distro", "sys"))
        .body(body)
        .blob(blob)
        .send_and_await_response(30)
        .map_err(|e| format!("Failed to send request: {}", e))?
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let blob = get_blob().ok_or("Failed to get response blob".to_string())?;

    let response: Value = serde_json::from_slice(&blob.bytes)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to extract content from response".to_string())
        .map(String::from)
}

pub fn get_subtext(img_urls: Vec<String>, content: String) -> Result<String, String> {
    let images = process_image_urls(img_urls);
    let request_body = create_request_body(images, &content);
    send_request(request_body)
}
