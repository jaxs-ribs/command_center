use crate::get_typed_state;
use kinode_process_lib::println;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use kinode_process_lib::set_state;

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
            println!("TG Curator: Deserialization error: {:?}", e);
            e
        })
    })
    .unwrap_or_else(|| {
        println!("HQ: No saved state found or deserialization failed, using default State");
        State::default()
    })
}

impl State {
    pub fn save(&self) -> anyhow::Result<()> {
        match serde_json::to_vec(self) {
            Ok(serialized) => {
                set_state(&serialized);
                Ok(())
            }
            Err(e) => {
                println!("Error serializing state: {:?}", e);
                Err(anyhow::anyhow!("Error serializing state"))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TgCuratorRequest {
    CurateLink {
        stream_name: String,
        site: String,
        post_id: String,
    },
}
