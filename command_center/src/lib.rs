use crate::kinode::process::llm::{ChatRequest, EmbeddingRequest, LlmRequest, LlmResponse, Message as LlmMessage, ClaudeChatRequest};
use crate::kinode::process::stt::{openai_transcribe, register_api_key};
use kinode_process_lib::{call_init, println, Address, Request};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn _stt() -> anyhow::Result<()> {
    let result = register_api_key("u no taek key");
    println!("Result is going to be {:?}", result);

    let audio = vec![0; 1024];
    let result = openai_transcribe(&audio);
    println!("Result is going to be {:?}", result);

    Ok(())
}

fn _register_openai_api_key(api_key: &str) -> anyhow::Result<String> {
    let LlmResponse::RegisterOpenaiApiKey(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterOpenaiApiKey(api_key.to_string()))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register OpenAI API key: unexpected response"));
    };
    Ok(result)
}

fn _register_groq_api_key(api_key: &str) -> anyhow::Result<String> {
    let LlmResponse::RegisterGroqApiKey(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterGroqApiKey(api_key.to_string()))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register Groq API key: unexpected response"));
    };
    Ok(result)
}

fn _register_claude_api_key(api_key: &str) -> anyhow::Result<String> {
    let LlmResponse::RegisterClaudeApiKey(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterClaudeApiKey(api_key.to_string()))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register Claude API key: unexpected response"));
    };
    Ok(result)
}

fn _get_embedding(input: &str, model: Option<&str>) -> anyhow::Result<Vec<f32>> {
    let embedding_request = EmbeddingRequest {
        model: model.unwrap_or("text-embedding-3-large").to_string(),
        input: vec![input.to_string()],
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
    Ok(result.embeddings[0].clone())
}

fn _openai_chat(input: &str, model: Option<&str>) -> anyhow::Result<String> {
    let chat_request = ChatRequest {
        model: model.unwrap_or("gpt-3.5-turbo").to_string(),
        messages: vec![
            LlmMessage {
                role: "user".to_string(),
                content: input.to_string(),
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
    let LlmResponse::OpenaiChat(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::OpenaiChat(chat_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to get OpenAI chat response: unexpected response"));
    };
    Ok(result.choices[0].message.content.clone())
}

fn _groq_chat(input: &str, model: Option<&str>) -> anyhow::Result<String> {
    let chat_request = ChatRequest {
        model: model.unwrap_or("llama3-8b-8192").to_string(),
        messages: vec![
            LlmMessage {
                role: "user".to_string(),
                content: input.to_string(),
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
    let LlmResponse::GroqChat(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::GroqChat(chat_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to get Groq chat response: unexpected response"));
    };
    Ok(result.choices[0].message.content.clone())
}

fn _claude_chat(input: &str, model: Option<&str>) -> anyhow::Result<String> {
    let claude_chat_request = ClaudeChatRequest {
        model: model.unwrap_or("claude-3-5-sonnet-20240620").to_string(),
        messages: vec![
            LlmMessage {
                role: "user".to_string(),
                content: input.to_string(),
            },
        ],
        max_tokens: Some(512),
    };
    let LlmResponse::ClaudeChat(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::ClaudeChat(claude_chat_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to get Claude chat response: unexpected response"));
    };
    Ok(result.content[0].text.clone())
}

// Update the llm() function to demonstrate the use of optional model parameters
fn llm() -> anyhow::Result<()> {
    println!("Starting");

    // Register API keys
    println!("OpenAI API key registered: {:?}", _register_openai_api_key("sk-proj-J2y0MMBBYhLaw6iI680bT3BlbkFJPeH4fI3cumGNe6M6mbLX")?);
    println!("Groq API key registered: {:?}", _register_groq_api_key("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")?);
    println!("Claude API key registered: {:?}", _register_claude_api_key("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")?);

    // Get embedding
    let embedding = _get_embedding("this is an embedding test", None)?;
    println!("Embedding result len: {}", embedding.len());

    // Chat with different models
    println!("OpenAI chat result (default model): {}", _openai_chat("What is the capital of France?", None)?);
    println!("OpenAI chat result (GPT-4): {}", _openai_chat("What is the capital of France?", Some("gpt-4"))?);
    println!("Groq chat result (default model): {}", _groq_chat("What is the capital of Germany?", None)?);
    println!("Claude chat result (default model): {}", _claude_chat("What is the capital of Japan?", None)?);

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
