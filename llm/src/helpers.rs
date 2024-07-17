use crate::kinode::process::llm::ClaudeUsage;
use crate::kinode::process::llm::Content;
use crate::kinode::process::llm::ImageUrl;
use crate::kinode::process::llm::{
    ClaudeChatResponse, EmbeddingResponse, LlmRequest, OpenaiUsage, ChatImageContent
};
use serde::Deserialize;
use serde::Serialize;

pub const REGISTER_API_KEY_CONTEXT: u8 = 0;
pub const EMBEDDING_CONTEXT: u8 = 1;
pub const OPENAI_CHAT_CONTEXT: u8 = 2;
pub const GROQ_CHAT_CONTEXT: u8 = 3;
pub const CHAT_IMAGE_CONTEXT: u8 = 4;
pub const CLAUDE_CHAT_CONTEXT: u8 = 5;

// TODO: Zena: We should probably derive this through a trait at some point?
pub fn request_to_context(request: &LlmRequest) -> u8 {
    match request {
        LlmRequest::RegisterGroqApiKey(_)
        | LlmRequest::RegisterOpenaiApiKey(_)
        | LlmRequest::RegisterClaudeApiKey(_) => REGISTER_API_KEY_CONTEXT,
        LlmRequest::Embedding(_) => EMBEDDING_CONTEXT,
        LlmRequest::OpenaiChat(_) => OPENAI_CHAT_CONTEXT,
        LlmRequest::GroqChat(_) => GROQ_CHAT_CONTEXT,
        LlmRequest::ChatImage(_) => CHAT_IMAGE_CONTEXT,
        LlmRequest::ClaudeChat(_) => CLAUDE_CHAT_CONTEXT,
    }
}

pub fn serialize_without_none<T: Serialize>(input: &T) -> serde_json::Result<Vec<u8>> {
    let mut value = serde_json::to_value(input)?;
    if let serde_json::Value::Object(ref mut map) = value {
        map.retain(|_, v| !v.is_null());
    }
    serde_json::to_vec(&value)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiEmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: OpenaiUsage,
}

impl OpenAiEmbeddingResponse {
    pub fn to_embedding_response(&self) -> EmbeddingResponse {
        let embedding_values: Vec<Vec<f32>> = self
            .data
            .iter()
            .map(|embedding_data| {
                embedding_data
                    .embedding
                    .iter()
                    .map(|&value| value as f32)
                    .collect()
            })
            .collect();
        EmbeddingResponse {
            embeddings: embedding_values,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmbeddingData {
    pub object: String,
    pub index: u32,
    pub embedding: Vec<f64>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ChatImageContentWrapper {
    #[serde(rename = "type")]
    data_type: String,
    text: Option<String>,
    image_url: Option<ImageUrl>,
}

impl From<ChatImageContentWrapper> for ChatImageContent {
    fn from(wrapper: ChatImageContentWrapper) -> Self {
        ChatImageContent {
            data_type: wrapper.data_type,
            text: wrapper.text,
            image_url: wrapper.image_url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClaudeChatResponseWrapper {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    role: String,
    model: String,
    content: Vec<ContentWrapper>,
    stop_reason: String,
    stop_sequence: Option<String>,
    usage: ClaudeUsage,
}

impl From<ClaudeChatResponseWrapper> for ClaudeChatResponse {
    fn from(wrapper: ClaudeChatResponseWrapper) -> Self {
        ClaudeChatResponse {
            content: wrapper.content.into_iter().map(Content::from).collect(),
            id: wrapper.id,
            model: wrapper.model,
            role: wrapper.role,
            stop_reason: wrapper.stop_reason,
            stop_sequence: wrapper.stop_sequence,
            data_type: wrapper.message_type,
            usage: wrapper.usage,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentWrapper {
    #[serde(rename = "type")]
    data_type: String,
    text: String,
}

impl From<ContentWrapper> for Content {
    fn from(wrapper: ContentWrapper) -> Self {
        Content {
            data_type: wrapper.data_type,
            text: wrapper.text,
        }
    }
}