use serde::{Deserialize, Serialize};
use process_macros::SerdeJsonInto;
use std::collections::HashMap;

pub type ContentHash = String;
pub type Content = String;
pub type Embedding = Vec<f32>;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct State {
    pub embedding_hash_map: HashMap<ContentHash, Embedding>,

}

#[derive(Debug, Serialize, Deserialize, SerdeJsonInto, Clone)]
pub enum RecenteredRequest {
    GetEmbeddingsForTexts(Vec<String>),
    FilterPostsWithRules { rules: Vec<String>, post_contents: Vec<String>, }
}

#[derive(Debug, Serialize, Deserialize, SerdeJsonInto, Clone)]
pub enum RecenteredResponse {
    GetEmbeddingsForTexts(Result<Vec<Vec<f32>>, String>),
    FilterPostsWithRules(Result<Vec<bool>, String>)
}