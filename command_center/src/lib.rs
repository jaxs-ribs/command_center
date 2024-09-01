use std::collections::HashMap;

use crate::kinode::process::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::kinode::process::llm::{embedding, register_openai_api_key};
use kinode_process_lib::{
    await_message, call_init, get_typed_state, println, set_state, Address, Message, Response,
};
use sha2::{Digest, Sha256};

const OPENAI_API_KEY: &str = include_str!("../../OPENAI_API_KEY");

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

pub type ContentHash = String;
pub type Content = String;
pub type Embedding = Vec<f32>;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct State {
    pub master_hash_map: HashMap<ContentHash, Embedding>,

    pub incoming_hashes: Vec<ContentHash>,
    pub new_hashes: Vec<ContentHash>,
    pub content_to_embed: Vec<Content>,
}

fn handle_request(state: &mut State, body: &[u8], source: &Address) -> anyhow::Result<()> {
    let request: EmbeddingRequest = serde_json::from_slice(body)?;
    match request {
        EmbeddingRequest::GetEmbeddingsForTexts(texts) => {
            handle_embedding_request(state, texts, source)
        }
    }
}

fn handle_embedding_request(
    state: &mut State,
    texts: Vec<String>,
    source: &Address,
) -> anyhow::Result<()> {
    println!("Received embedding request from {:?}", source);
    println!("Incoming text length is {} ", texts.len());
    for text in &texts {
        let content_hash = content_hash(text);
        state.incoming_hashes.push(content_hash.clone());
        if !state.master_hash_map.contains_key(&content_hash) {
            state.new_hashes.push(content_hash);
            state.content_to_embed.push(text.clone());
        }
    }

    if !state.content_to_embed.is_empty() {
        let Ok(new_embeddings) = embedding(&state.content_to_embed, None) else {
            return Err(anyhow::anyhow!("Failed to get embeddings"));
        };

        assert_eq!(state.new_hashes.len(), new_embeddings.len());
        for (hash, embedding) in state.new_hashes.iter().zip(new_embeddings.iter()) {
            state
                .master_hash_map
                .insert(hash.clone(), embedding.clone());
        }
    }

    println!("The non existing hashes are: {}", state.new_hashes.len());
    println!("The amount of existing hashes is: {}", state.incoming_hashes.len() - state.new_hashes.len());
    let mut return_list = Vec::new();
    for hash in state.incoming_hashes.iter() {
        return_list.push(state.master_hash_map.get(hash).unwrap().clone());
    }

    state.incoming_hashes.clear();
    state.new_hashes.clear();
    state.content_to_embed.clear();

    let response = EmbeddingResponse::GetEmbeddingsForTexts(Ok(return_list));
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

fn handle_message(state: &mut State, _our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;
    if let Message::Request { body, source, .. } = message {
        handle_request(state, &body, &source)?;
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("Starting command centers embedding engine");
    println!("{:?}", register_openai_api_key(OPENAI_API_KEY).unwrap());

    let mut state: State =
        get_typed_state(|bytes| bincode::deserialize(bytes).map_err(Box::new)).unwrap_or_default();

    loop {
        if let Err(e) = handle_message(&mut state, &our) {
            println!("Error: {:?}", e);
        }
    }
}
