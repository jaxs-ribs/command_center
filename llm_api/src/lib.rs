use crate::exports::kinode::process::llm::{
    ChatRequest, ClaudeChatRequest, EmbeddingRequest, Guest, LlmRequest, LlmResponse,
    Message as LlmMessage,
};
use kinode_process_lib::{vfs, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "llm-uncentered-dot-os-api-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn _register_openai_api_key(api_key: &str) -> anyhow::Result<String> {
    let LlmResponse::RegisterOpenaiApiKey(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterOpenaiApiKey(api_key.to_string()))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!(
            "Failed to register OpenAI API key: unexpected response"
        ));
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
        return Err(anyhow::anyhow!(
            "Failed to register Groq API key: unexpected response"
        ));
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
        return Err(anyhow::anyhow!(
            "Failed to register Claude API key: unexpected response"
        ));
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
        return Err(anyhow::anyhow!(
            "Failed to get embedding: unexpected response"
        ));
    };
    Ok(result.embeddings[0].clone())
}

fn _openai_chat(input: &str, model: Option<&str>) -> anyhow::Result<String> {
    let chat_request = ChatRequest {
        model: model.unwrap_or("gpt-3.5-turbo").to_string(),
        messages: vec![LlmMessage {
            role: "user".to_string(),
            content: input.to_string(),
        }],
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
        return Err(anyhow::anyhow!(
            "Failed to get OpenAI chat response: unexpected response"
        ));
    };
    Ok(result.choices[0].message.content.clone())
}

fn _groq_chat(input: &str, model: Option<&str>) -> anyhow::Result<String> {
    let chat_request = ChatRequest {
        model: model.unwrap_or("llama3-8b-8192").to_string(),
        messages: vec![LlmMessage {
            role: "user".to_string(),
            content: input.to_string(),
        }],
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
        return Err(anyhow::anyhow!(
            "Failed to get Groq chat response: unexpected response"
        ));
    };
    Ok(result.choices[0].message.content.clone())
}

fn _claude_chat(input: &str, model: Option<&str>) -> anyhow::Result<String> {
    let claude_chat_request = ClaudeChatRequest {
        model: model.unwrap_or("claude-3-5-sonnet-20240620").to_string(),
        messages: vec![LlmMessage {
            role: "user".to_string(),
            content: input.to_string(),
        }],
        max_tokens: Some(512),
    };
    let LlmResponse::ClaudeChat(Ok(result)) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::ClaudeChat(claude_chat_request))
        .send_and_await_response(20)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!(
            "Failed to get Claude chat response: unexpected response"
        ));
    };
    Ok(result.content[0].text.clone())
}

struct Api;
impl Guest for Api {
    fn register_openai_api_key(key: String) -> Result<String, String> {
        match _register_openai_api_key(&key) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string()),
        }
    }

    fn register_groq_api_key(key: String) -> Result<String, String> {
        match _register_groq_api_key(&key) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string()),
        }
    }

    fn register_claude_api_key(key: String) -> Result<String, String> {
        match _register_claude_api_key(&key) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string()),
        }
    }

    fn embedding(input: String, model: Option<String>) -> Result<String, String> {
        match _get_embedding(&input, model.as_deref()) {
            Ok(v) => Ok(serde_json::to_string(&v).unwrap()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn openai_chat(input: String, model: Option<String>) -> Result<String, String> {
        match _openai_chat(&input, model.as_deref()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string()),
        }
    }

    fn groq_chat(input: String, model: Option<String>) -> Result<String, String> {
        match _groq_chat(&input, model.as_deref()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string()),
        }
    }

    fn claude_chat(input: String, model: Option<String>) -> Result<String, String> {
        match _claude_chat(&input, model.as_deref()) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string()),
        }
    }
}
export!(Api);
