use crate::content_hash;
use crate::State;
use anyhow;
use kinode_process_lib::{
    get_blob,
    http::client::{HttpClientAction, OutgoingHttpRequest},
    println, set_state, LazyLoadBlob, Request, Address,
};
use serde::Deserialize;
use serde_json;
use serde_json::json;
use std::collections::HashMap;

const EMBED_URL: &str = "https://ai.sortug.com/embed";

#[derive(Deserialize)]
struct EmbeddingResponse {
    embeddings: Vec<Vec<f32>>,
    time_taken: f32,
}

fn get_embeddings(texts: &[String], is_query: bool) -> anyhow::Result<Vec<Vec<f32>>> {
    let request_body = json!({
        "texts": texts,
        "is_query": is_query
    });

    let headers = HashMap::from_iter(vec![(
        "Content-Type".to_string(),
        "application/json".to_string(),
    )]);

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

    let response: EmbeddingResponse = serde_json::from_slice(&blob.bytes.as_slice())?;

    // Optionally log the time taken
    println!("Embedding time taken: {} seconds", response.time_taken);

    Ok(response.embeddings)
}

pub fn get_embeddings_for_text(
    state: &mut State,
    texts: Vec<String>,
    is_query: bool,
    _source: &Address,
) -> Result<Vec<Vec<f32>>, String> {
    // println!("Node {:?} is requesting embeddings for {:?} texts ", _source, texts.len());
    let mut input_hashes = Vec::new();
    let mut unembedded_hashes = Vec::new();
    let mut contents_to_embed = Vec::new();
    let mut return_list = Vec::new();

    for text in &texts {
        let content_hash = content_hash(text);
        input_hashes.push(content_hash.clone());
        if !state.embedding_hash_map.contains_key(&content_hash) {
            unembedded_hashes.push(content_hash);
            contents_to_embed.push(text.clone());
        }
    }

    for (unembedded_hash, content_to_embed) in
        unembedded_hashes.iter().zip(contents_to_embed.iter())
    {
        println!("Content to embed: {:?}", content_to_embed);
        println!("----------------------------------");
        let embedding = get_embedding(content_to_embed, is_query)?;
        state.embedding_hash_map.insert(unembedded_hash.to_string(), embedding);
    }

    for hash in input_hashes.iter() {
        return_list.push(state.embedding_hash_map.get(hash).unwrap().clone());
    }
    
    match bincode::serialize(&state) {
        Ok(serialized) => set_state(&serialized),
        Err(e) => return Err(format!("Failed to serialize state: {}", e)),
    }

    Ok(return_list)
}

fn get_embedding(text: &str, is_query: bool) -> Result<Vec<f32>, String> {
    let new_embeddings = match get_embeddings(&[text.to_string()], is_query) {
        Ok(embeddings) => embeddings,
        Err(e) => {
            println!("Error obtaining embeddings: {}", e);
            return Err(format!("Failed to get embeddings: {}", e));
        }
    };
    Ok(new_embeddings[0].clone())
}
