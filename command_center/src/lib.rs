use crate::kinode::process::llm::{register_groq_api_key, register_openai_api_key};
use crate::kinode::process::embedding::{EmbeddingRequest, EmbeddingResponse};
use kinode_process_lib::{await_message, call_init, get_blob, println, Address, Message};

const OPENAI_API_KEY: &str = include_str!("../../OPENAI_API_KEY");

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

pub struct State {

}

fn handle_request(body: &[u8]) -> Result<(), String> {
    let request: EmbeddingRequest = serde_json::from_slice(body)?;
    match request {
        EmbeddingRequest::GetEmbeddingsForTexts(texts) => get_embeddigns_for_texts(texts),
    }
    Ok(())
}

fn get_embeddigns_for_texts(texts: Vec<String>) -> Result<Vec<Vec<f32>>, String> {

}

fn handle_message(_our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;
    if let Message::Request { body, .. } = message {
        handle_request(&body)?;
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("Starting command centers embedding engine");
    println!("{:?}", register_openai_api_key(OPENAI_API_KEY).unwrap());

    loop {
        if let Err(e) = handle_message(&our) {
            println!("Error: {:?}", e);
        }
    }
}
