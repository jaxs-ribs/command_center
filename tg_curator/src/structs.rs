use serde::{Deserialize, Serialize};
use kinode_process_lib::println;
use crate::get_typed_state;
use std::collections::HashMap;


type ChatId = i64;
type KinodeAddress = String;
type Code = u64;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub address_book: HashMap<ChatId, KinodeAddress>,
    pub pending_codes: HashMap<ChatId, (KinodeAddress, Code)>,
}

pub fn load_state() -> State {
    get_typed_state::<State, _, serde_json::Error>(|bytes| {
        serde_json::from_slice(bytes).map_err(|e| {
            println!("HQ: Deserialization error: {:?}", e);
            e
        })
    })
    .unwrap_or_else(|| {
        println!(
            "HQ: No saved state found or deserialization failed, using default OffchainState"
        );
        State::default()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TgCuratorRequest {
    CurateLink { stream_name: String, site: String, post_id: String },
}