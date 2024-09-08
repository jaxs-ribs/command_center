use crate::content_hash;
use crate::kinode::process::llm::embedding;
use crate::State;
use kinode_process_lib::{println, set_state, Address};

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
        let new_embeddings = match embedding(&content_to_embed, None) {
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
