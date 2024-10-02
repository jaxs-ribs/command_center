use process_macros::SerdeJsonInto;
use serde::{Deserialize, Serialize};
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
    GetEmbeddingsForTexts {
        texts: Vec<String>,
        is_query: bool,
    },
    FilterPostsWithRules {
        rules: Vec<String>,
        post_contents: Vec<String>,
    },
    GetDescriptionFromMedia {
        img_urls: Vec<String>,
        post_uuid: String,
        stream_uuid: String,
    },
}

#[derive(Debug, Serialize, Deserialize, SerdeJsonInto, Clone)]
pub enum RecenteredResponse {
    GetEmbeddingsForTexts(Result<Vec<Vec<f32>>, String>),
    FilterPostsWithRules(Result<Vec<bool>, String>),
    GetDescriptionFromMedia(Result<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, process_macros::SerdeJsonInto)]
pub enum ImgServerRequest {
    UploadImage,
    GetImage(URI),
}

#[derive(Debug, Clone, Serialize, Deserialize, process_macros::SerdeJsonInto)]
pub enum ImgServerResponse {
    UploadImage(Result<URI, String>),
    GetImage(Result<String, String>),
}

pub type URI = String;
