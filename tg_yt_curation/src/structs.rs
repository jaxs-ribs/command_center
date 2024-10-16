use serde::{Deserialize, Serialize};
use kinode_process_lib::Address;
use kinode_process_lib::{set_state, get_typed_state};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub enum TGMessage {
    Start(),
    Register(String),
    Authenticate(u64),
    CurationMSGToEmbedLinkRequest(String),
    Unknown(String),
}
// Response enum
#[derive(Debug, Serialize, Deserialize)]
pub enum TelegramYoutubeCurationResponse {
    Success(String),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TGYoutubeCurationMessage {
	pub share_link: String, //This would have video_id and start_time, but not end_time
    pub start_time: Option<String>,
	pub duration: Option<String>, // default 30s
	pub curation_quote: Option<String>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeEmbedParams {
	pub video_id: String,
	pub start_time: String,
	pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeCuration {
	pub embed_params: YoutubeEmbedParams,
    pub curation_quote: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub pending_codes: HashMap<String, (String, u64)>, // chat_id -> (kinode_address, code)
    pub address_book: HashMap<String, String>, // chat_id -> kinode_address
}

pub fn load_state() -> State {
    get_typed_state::<State, _, serde_json::Error>(|bytes| {
        serde_json::from_slice(bytes).map_err(|e| {
            println!("TG YT Curator: Deserialization error: {:?}", e);
            e
        })
    })
    .unwrap_or_else(|| {
        println!("HQ: No saved state found or deserialization failed, using default State");
        State {
            pending_codes: HashMap::new(),
            address_book: HashMap::new(),
        }
    })
}
//pub fn load_state() -> State {
    //get_typed_state::<State, _, serde_json::Error>(|bytes| {
        //serde_json::from_slice(bytes).map_err(|e| {
            //println!("TG YT Curator: Deserialization error: {:?}", e);
            //e
        //})
    //})
    //.unwrap_or_else(|| {
        //println!("TG YT Curator: No saved state found or deserialization failed, using default State");
        //State {
            //pending_codes: HashMap::new(),
            //address_book: HashMap::new(),
        //}
    //})
//}


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