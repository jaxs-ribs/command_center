use std::collections::HashMap;

use crate::kinode::process::llm::{register_groq_api_key, register_openai_api_key};
use crate::kinode::process::embedding::{EmbeddingRequest, EmbeddingResponse};
use kinode_process_lib::{await_message, call_init, get_blob, get_typed_state, println, Address, Message};
use sha2::{Sha256, Digest};

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

fn handle_request(state: &mut State, body: &[u8]) -> anyhow::Result<()> {
    let request: EmbeddingRequest = serde_json::from_slice(body)?;
    match request {
        EmbeddingRequest::GetEmbeddingsForTexts(texts) => handle_embedding_request(state, texts),
    }
}

fn handle_embedding_request(state: &mut State, texts: Vec<String>) -> anyhow::Result<()> {
    for text in &texts {
        let content_hash = content_hash(text);
        state.incoming_hashes.push(content_hash.clone());
        if state.master_hash_map.contains_key(&content_hash) {
            state.new_hashes.push(content_hash);
            state.content_to_embed.push(text.clone());
        }

        // let new_embeddings = get_embeddings();
        

    }



    // TODO: Send response
    Ok(())
}


fn content_hash(content: &Content) -> ContentHash {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}


fn handle_message(state: &mut State, _our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;
    if let Message::Request { body, .. } = message {
        handle_request(state, &body)?;
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("Starting command centers embedding engine");
    println!("{:?}", register_openai_api_key(OPENAI_API_KEY).unwrap());

    let mut state: State =
        get_typed_state(|bytes| Ok(bincode::deserialize(bytes)?)).unwrap_or_default();

    loop {
        if let Err(e) = handle_message(&mut state, &our) {
            println!("Error: {:?}", e);
        }
    }
}
