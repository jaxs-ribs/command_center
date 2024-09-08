use crate::kinode::process::llm::embedding;
use crate::Content;
use crate::ContentHash;
use crate::RecenteredResponse;
use crate::State;
use kinode_process_lib::{println, set_state, Address, Response};
use sha2::Sha256;
use sha2::Digest;

pub fn handle_embedding_request(
    state: &mut State,
    texts: Vec<String>,
    source: &Address,
) -> anyhow::Result<()> {
    println!("Received embedding request from {:?}", source);
    println!("Incoming text length is {} ", texts.len());
    for text in &texts {
        if text.is_empty() {
            println!("Skipping empty text");
            continue;
        }
        let content_hash = content_hash(text);
        state.incoming_hashes.push(content_hash.clone());
        if !state.master_hash_map.contains_key(&content_hash) {
            state.new_hashes.push(content_hash);
            state.content_to_embed.push(text.clone());
        }
    }

    if !state.content_to_embed.is_empty() {
        for content in &state.content_to_embed {
            println!("Embedding content: {}", content);
        }
        let new_embeddings = match embedding(&state.content_to_embed, None) {
            Ok(embeddings) => embeddings,
            Err(e) => return Err(anyhow::anyhow!("Failed to get embeddings: {}", e)),
        };

        assert_eq!(state.new_hashes.len(), new_embeddings.len());
        for (hash, embedding) in state.new_hashes.iter().zip(new_embeddings.iter()) {
            state
                .master_hash_map
                .insert(hash.clone(), embedding.clone());
        }
    }

    println!("The non existing hashes are: {}", state.new_hashes.len());
    println!(
        "The amount of existing hashes is: {}",
        state.incoming_hashes.len() - state.new_hashes.len()
    );
    let mut return_list = Vec::new();
    for hash in state.incoming_hashes.iter() {
        return_list.push(state.master_hash_map.get(hash).unwrap().clone());
    }

    state.incoming_hashes.clear();
    state.new_hashes.clear();
    state.content_to_embed.clear();

    let response = RecenteredResponse::GetEmbeddingsForTexts(Ok(return_list));
    Response::new()
        .body(serde_json::to_vec(&response)?)
        .send()?;

    set_state(&bincode::serialize(&state)?);

    Ok(())
}

fn content_hash(content: &Content) -> ContentHash {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}
