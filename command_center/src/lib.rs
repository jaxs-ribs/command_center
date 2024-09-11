use crate::kinode::process::llm::{register_groq_api_key, register_openai_api_key};
use kinode_process_lib::{
    await_message, call_init, get_typed_state, println, Address, Message, Response,
};

const OPENAI_API_KEY: &str = include_str!("../../OPENAI_API_KEY");
const GROQ_API_KEY: &str = include_str!("../../GROQ_API_KEY");

mod structs;
use structs::*;

mod llm_filter;
use llm_filter::*;

mod embedding;
use embedding::*;

mod helpers;
use helpers::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_request(state: &mut State, body: &[u8], source: &Address) -> anyhow::Result<()> {
    let request: RecenteredRequest = serde_json::from_slice(body)?;
    match request {
        RecenteredRequest::GetEmbeddingsForTexts { texts, is_query } => {
            handle_get_embeddings_for_texts(state, texts, source, is_query)
        }
        RecenteredRequest::FilterPostsWithRules {
            rules,
            post_contents,
        } => handle_filter_posts_with_rules(rules, post_contents),
    }
}

fn handle_get_embeddings_for_texts(
    state: &mut State,
    texts: Vec<String>,
    source: &Address,
    is_query: bool,
) -> anyhow::Result<()> {
    let return_list = get_embeddings_for_text(state, texts, source, is_query);
    let response = RecenteredResponse::GetEmbeddingsForTexts(return_list);
    Ok(Response::new()
        .body(serde_json::to_vec(&response)?)
        .send()?)
}

fn handle_filter_posts_with_rules(
    rules: Vec<String>,
    post_contents: Vec<String>,
) -> anyhow::Result<()> {
    let result = filter_posts(rules, post_contents);
    let response = RecenteredResponse::FilterPostsWithRules(result);
    Ok(Response::new()
        .body(serde_json::to_vec(&response)?)
        .send()?)
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
    println!("{:?}", register_groq_api_key(GROQ_API_KEY).unwrap());

    let mut state: State =
        get_typed_state(|bytes| bincode::deserialize(bytes).map_err(Box::new)).unwrap_or_default();
    // let mut state = State::default(); // TODO: Remove this

    loop {
        if let Err(e) = handle_message(&mut state, &our) {
            println!("Error: {:?}", e);
        }
    }
}
