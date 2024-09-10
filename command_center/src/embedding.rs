use crate::content_hash;
use crate::kinode::process::llm::embedding;
use crate::State;
use kinode_process_lib::{
    println, set_state, Address,
    http::client::{HttpClientAction, OutgoingHttpRequest},
    Request, LazyLoadBlob,
};
use serde_json::json;
use std::collections::HashMap;
use anyhow;
use serde_json;

pub fn get_embeddings_for_text(
    state: &mut State,
    texts: Vec<String>,
    source: &Address,
) -> Result<Vec<Vec<f32>>, String> {
    println!("Received embedding request from {:?}", source);
    println!("Incoming text length is {} ", texts.len());

    let mut incoming_hashes = Vec::new();
    let mut new_hashes = Vec::new();
    let mut content_to_embed = Vec::new();

    for text in &texts {
        if text.is_empty() {
            println!("Skipping empty text");
            continue;
        }
        let content_hash = content_hash(text);
        incoming_hashes.push(content_hash.clone());
        if !state.embedding_hash_map.contains_key(&content_hash) {
            new_hashes.push(content_hash);
            content_to_embed.push(text.clone());
        }
    }

    if !content_to_embed.is_empty() {
        for content in &content_to_embed {
            println!("Embedding content: {}", content);
        }
        // let new_embeddings = match embedding(&content_to_embed, None) {
        let new_embeddings = match nvidia_embedding(&content_to_embed) {
            Ok(embeddings) => embeddings,
            Err(e) => return Err(format!("Failed to get embeddings: {}", e)),
        };

        assert_eq!(new_hashes.len(), new_embeddings.len());
        for (hash, embedding) in new_hashes.iter().zip(new_embeddings.iter()) {
            state
                .embedding_hash_map
                .insert(hash.clone(), embedding.clone());
        }
    }

    println!("The non existing hashes are: {}", new_hashes.len());
    println!(
        "The amount of existing hashes is: {}",
        incoming_hashes.len() - new_hashes.len()
    );
    let mut return_list = Vec::new();
    for hash in incoming_hashes.iter() {
        return_list.push(state.embedding_hash_map.get(hash).unwrap().clone());
    }
    match bincode::serialize(&state) {
        Ok(serialized) => set_state(&serialized),
        Err(e) => return Err(format!("Failed to serialize state: {}", e)),
    }
    Ok(return_list)
}


const NVIDIA_EMBED_URL: &str = "https://integrate.api.nvidia.com/v1/embeddings";
const NVIDIA_API_KEY: &str = "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";

fn nvidia_embedding(texts: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
    let request_body = json!({
        "input": texts,
        "model": "nvidia/nv-embed-v1",
        "input_type": "query",
        "encoding_format": "float",
        "truncate": "NONE"
    });

    let headers = HashMap::from_iter(vec![
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Authorization".to_string(), format!("Bearer {}", NVIDIA_API_KEY)),
    ]);

    let outgoing_request = OutgoingHttpRequest {
        method: "POST".to_string(),
        version: None,
        url: NVIDIA_EMBED_URL.to_string(),
        headers,
    };

    let body = serde_json::to_vec(&HttpClientAction::Http(outgoing_request))?;
    let blob = LazyLoadBlob {
        mime: Some("application/json".to_string()),
        bytes: serde_json::to_vec(&request_body)?,
    };

    let response = Request::new()
        .target(("our", "http_client", "distro", "sys"))
        .body(body)
        .blob(blob)
        .send_and_await_response(5)??;

    let response_body: serde_json::Value = serde_json::from_slice(response.body())?;
    let embeddings = response_body["data"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
        .iter()
        .map(|item| {
            item["embedding"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Invalid embedding format"))?
                .iter()
                .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid float value")).map(|f| f as f32))
                .collect::<Result<Vec<f32>, _>>()
        })
        .collect::<Result<Vec<Vec<f32>>, _>>()?;

    Ok(embeddings)
}
/*

```bash
curl -X POST https://integrate.api.nvidia.com/v1/embeddings \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer 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" \
  -d '{
    "input": ["What is the capital of France?"],
    "model": "nvidia/nv-embed-v1",
    "input_type": "query",
    "encoding_format": "float",
    "truncate": "NONE"
  }'
```
*/