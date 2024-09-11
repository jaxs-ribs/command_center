use crate::content_hash;
use crate::State;
use kinode_process_lib::{
    println, set_state, Address,
    http::client::{HttpClientAction, OutgoingHttpRequest},
    Request, LazyLoadBlob,
    get_blob,
};
use serde_json::json;
use std::collections::HashMap;
use anyhow;
use serde_json;

const EMBED_URL: &str = "https://ai.sortug.com/embed";

fn get_embeddings(texts: &[String], is_query: bool) -> anyhow::Result<Vec<Vec<f32>>> {
    let request_body = json!({
        "texts": texts,
        "is_query": is_query
    });

    let headers = HashMap::from_iter(vec![
        ("Content-Type".to_string(), "application/json".to_string()),
    ]);

    let outgoing_request = OutgoingHttpRequest {
        method: "POST".to_string(),
        version: None,
        url: EMBED_URL.to_string(),
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
        .send_and_await_response(15)??;

    let Some(blob) = get_blob() else {
        return Err(anyhow::anyhow!("Failed to get blob").into());
    };
    let embeddings: Vec<Vec<f32>> = serde_json::from_slice(&blob.bytes.as_slice())?;

    Ok(embeddings)
}

pub fn get_embeddings_for_text(
    state: &mut State,
    texts: Vec<String>,
    source: &Address,
    is_query: bool,
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
        let new_embeddings = match get_embeddings(&content_to_embed, is_query) {
            Ok(embeddings) => embeddings,
            Err(e) => {
                println!("Error obtaining embeddings: {}", e);
                return Err(format!("Failed to get embeddings: {}", e))
            },
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