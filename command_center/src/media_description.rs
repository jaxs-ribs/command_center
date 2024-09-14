use serde_json::json;
use kinode_process_lib::{
    http::client::{HttpClientAction, OutgoingHttpRequest},
    Request, LazyLoadBlob,
    get_blob,
};
use std::collections::HashMap;
use serde_json::Value;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_API_KEY: &str = include_str!("../../OPENAI_API_KEY");

pub fn get_description_from_media(
    img_url: String,
) -> Result<String, String> {
    let request_body = json!({
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Look at this image. You have to generate a textual description that tries to capture as much semantic meaning about this picture as possible. Your answer will be embedded by a text model (ie converted to a vector), and then a text query will be vectorized, and we want to find the nearest matches. Imagine someone wants to find a meme again, and you need to capture semantic understanding of the meme too, even if they give a vague query. Answer only with the description."
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": img_url
                        }
                    }
                ]
            }
        ],
        "max_tokens": 600
    });

    let headers = HashMap::from_iter(vec![
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Authorization".to_string(), format!("Bearer {}", OPENAI_API_KEY)),
    ]);

    let outgoing_request = OutgoingHttpRequest {
        method: "POST".to_string(),
        version: None,
        url: OPENAI_API_URL.to_string(),
        headers,
    };

    let body = match serde_json::to_vec(&HttpClientAction::Http(outgoing_request)) {
        Ok(b) => b,
        Err(e) => return Err(format!("Failed to serialize request: {}", e)),
    };

    let blob = LazyLoadBlob {
        mime: Some("application/json".to_string()),
        bytes: match serde_json::to_vec(&request_body) {
            Ok(b) => b,
            Err(e) => return Err(format!("Failed to serialize request body: {}", e)),
        },
    };

    let _response = match Request::new()
        .target(("our", "http_client", "distro", "sys"))
        .body(body)
        .blob(blob)
        .send_and_await_response(30)
    {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => return Err(format!("HTTP request failed: {}", e)),
        Err(e) => return Err(format!("Failed to send request: {}", e)),
    };

    let blob = match get_blob() {
        Some(b) => b,
        None => return Err("Failed to get response blob".to_string()),
    };

    let response: Value = match serde_json::from_slice(&blob.bytes) {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to parse response: {}", e)),
    };

    let content = response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to extract content from response")?
        .to_string();

    Ok(content)
}
