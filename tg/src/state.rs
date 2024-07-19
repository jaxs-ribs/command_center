use serde::{Deserialize, Serialize};
use kinode_process_lib::{get_state, set_state, Address};
use crate::Api;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct State {
    pub tg_key: String,
    pub api_url: String,
    pub current_offset: u32,
    pub subscribers: Vec<Address>,
    pub api: Option<Api>,
    pub our_channel_id: u32,
}


impl State {
    pub fn fetch() -> State {
        if let Some(state_bytes) = get_state() {
            match bincode::deserialize(&state_bytes) {
                Ok(state) => state,
                Err(_) => State::initialize_empty(),
            }
        } else {
            State::initialize_empty()
        }
    }

    pub fn save(&self) {
        let serialized_state = bincode::serialize(self).expect("Failed to serialize state");
        set_state(&serialized_state);
    }

    pub fn initialize_empty() -> State {
        State {
            tg_key: String::new(),
            api_url: String::new(),
            current_offset: 0,
            subscribers: Vec::new(),
            api: None,
            our_channel_id: 0,
        }
    }
}
