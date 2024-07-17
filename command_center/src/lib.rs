use crate::kinode::process::llm::{ChatRequest, EmbeddingRequest, LlmRequest, LlmResponse, Message as LlmMessage, ClaudeChatRequest};
use crate::kinode::process::stt::{openai_transcribe, register_api_key, SttRequest, SttResponse};
use kinode_process_lib::{await_message, call_init, println, Address, Message, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn stt() -> anyhow::Result<()> {
    let result = register_api_key("u no taek key");
    println!("Result is going to be {:?}", result);

    let audio = vec![0; 1024];
    let result = openai_transcribe(&audio);
    println!("Result is going to be {:?}", result);

    Ok(())
}

fn llm() -> anyhow::Result<()> {
    // register openai
    println!("Starting");
    let LlmResponse::RegisterOpenaiApiKey(result) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterOpenaiApiKey(
            "sk-proj-J2y0MMBBYhLaw6iI680bT3BlbkFJPeH4fI3cumGNe6M6mbLX".to_string(),
        ))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register OpenAI API key: unexpected response"));
    };
    println!("openai result: {:?}", result);
    println!("-----------------");

    // register groq
    let LlmResponse::RegisterGroqApiKey(result) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterGroqApiKey(
            "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".to_string(),
        ))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register Groq API key: unexpected response"));
    };
    println!("groq result: {:?}", result);
    println!("-----------------");

    // register claude
    let LlmResponse::RegisterClaudeApiKey(result) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterClaudeApiKey("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".to_string()))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register Claude API key: unexpected response"));
    };
    println!("claude result: {:?}", result);
    println!("-----------------");

    // Call the embedding method for openai
    let embedding_request = EmbeddingRequest {
        model: "text-embedding-3-large".to_string(),
        input: vec!["this is an embedding test".to_string()],
    };
    let LlmResponse::Embedding(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::Embedding(embedding_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to get embedding: unexpected response"));
    };
    println!("Embedding result len with openai: {:?}", result.embeddings[0].len());
    println!("-----------------");

    // Call the chat method for openai
    let chat_request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            LlmMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            LlmMessage {
                role: "user".to_string(),
                content: "What is the capital of France?".to_string(),
            },
        ],
        frequency_penalty: None,
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        max_tokens: None,
        n: None,
        presence_penalty: None,
        response_format: None,
        seed: None,
        stop: None,
        stream: None,
        temperature: None,
        top_p: None,
        tools: None,
        tool_choice: None,
        user: None,
        
    };
    let LlmResponse::OpenaiChat(result) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::OpenaiChat(chat_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to get OpenAI chat response: unexpected response"));
    };
    println!("Openai chat result: {:?}", result);
    println!("-----------------");

    // Call the chat method for groq
    let chat_request = ChatRequest {
        model: "llama3-8b-8192".to_string(),
        messages: vec![
            LlmMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            LlmMessage {
                role: "user".to_string(),
                content: "What is the capital of Germany?".to_string(),
            },
        ],
        frequency_penalty: None,
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        max_tokens: None,
        n: None,
        presence_penalty: None,
        response_format: None,
        seed: None,
        stop: None,
        stream: None,
        temperature: None,
        top_p: None,
        tools: None,
        tool_choice: None,
        user: None,
        
    };
    let LlmResponse::GroqChat(result) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::GroqChat(chat_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to get Groq chat response: unexpected response"));
    };
    println!("Groq chat result: {:?}", result);
    println!("-----------------");

    // Call the chat method for claude
    let claude_chat_request = ClaudeChatRequest {
        model: "claude-3-5-sonnet-20240620".to_string(),
        messages: vec![
            LlmMessage {
                role: "user".to_string(),
                content: "What is the capital of Germany?".to_string(),
            },
        ],
        max_tokens: Some(512),
    };
    let LlmResponse::ClaudeChat(result) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::ClaudeChat(claude_chat_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to get Claude chat response: unexpected response"));
    };
    println!("Claude chat result: {:?}", result);
    println!("-----------------");

    Ok(())
}

fn test() -> anyhow::Result<()> {
    // stt()?;
    llm()?;
    Ok(())
}

call_init!(init);
fn init(_our: Address) {
    println!("begin");
    match test() {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error: {e}"),
    }
}
