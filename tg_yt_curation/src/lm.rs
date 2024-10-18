use kinode_process_lib::{
    println,
    get_blob, Request, LazyLoadBlob,
    http::client::{HttpClientAction, OutgoingHttpRequest},
};
use serde_json::Value;
use std::collections::HashMap;
use crate::structs::TGYoutubeCurationMessage;

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const GROQ_API_KEY: &str = include_str!("../../GROQ_API_KEY");

const LM_INSTRUCTIONS: &str = include_str!("lm_instructions.md");

pub fn use_groq(msg: &str) -> anyhow::Result<TGYoutubeCurationMessage> {
    let request_body = serde_json::json!({
        "model": "mixtral-8x7b-32768",
        "messages": [
            {
                "role": "system",
                "content": LM_INSTRUCTIONS
            },
            {
                "role": "user",
                "content": msg
            }
        ],
        "max_tokens": 8192,
        "temperature": 0.0,
        "top_p": 0.9,
        "stream": false,
    });

    let headers = HashMap::from_iter(vec![
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Authorization".to_string(), format!("Bearer {}", GROQ_API_KEY)),
    ]);

    let outgoing_request = OutgoingHttpRequest {
        method: "POST".to_string(),
        version: None,
        url: GROQ_API_URL.to_string(),
        headers,
    };

    let body = serde_json::to_vec(&HttpClientAction::Http(outgoing_request))?;

    let blob = LazyLoadBlob {
        mime: Some("application/json".to_string()),
        bytes: serde_json::to_vec(&request_body)?,
    };

    let _response = Request::new()
        .target(("our", "http_client", "distro", "sys"))
        .body(body)
        .blob(blob)
        .send_and_await_response(30)??;

    let blob = get_blob().ok_or_else(|| anyhow::anyhow!("Failed to get response blob"))?;

    let response: Value = serde_json::from_slice(&blob.bytes)?;
    println!("Raw response: {:#?}", response);

    let content = response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            println!("Failed to extract content. Response structure: {:#?}", response);
            anyhow::anyhow!("Failed to extract content from response")
        })?;

    println!("Extracted content: {}", content);

    let json_start = content.find('{').ok_or_else(|| anyhow::anyhow!("Failed to find JSON start"))?;
    let json_end = content.rfind('}').ok_or_else(|| anyhow::anyhow!("Failed to find JSON end"))?;
    let json_str = &content[json_start..=json_end];

    let curation_message: TGYoutubeCurationMessage = serde_json::from_str(json_str)
    .map_err(|e| {
        println!("Failed to parse content into TGYoutubeCurationMessage. Error: {:?}", e);
        anyhow::anyhow!("Failed to parse content into TGYoutubeCurationMessage: {}", e)
    })?;

    Ok(curation_message)
}
