use crate::kinode::process::tg::{TgRequest, TgResponse};
use kinode_process_lib::{
    await_message, call_init, get_blob,
    http::{self, HttpClientAction, OutgoingHttpRequest},
    println, Address, Message, Request, Response,
};
use std::collections::HashMap;
use frankenstein::TelegramApi;
use frankenstein::GetFileParams;

mod state;
use state::*;

mod helpers;
use helpers::*;

static BASE_API_URL: &str = "https://api.telegram.org/bot";

wit_bindgen::generate!({
    path: "target/wit",
    world: "tg-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn send_response<T: serde::Serialize>(response: T) -> anyhow::Result<()> {
    Response::new()
        .body(serde_json::to_vec(&response)?)
        .send()
        .map_err(|e| anyhow::anyhow!("Failed to send response: {}", e))
}

fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<()> {
    let message = await_message()?;
    match message {
        Message::Request {
            ref body, source, ..
        } => handle_request(our, state, body, &source),
        Message::Response { .. } => Ok(()),
    }
}

// TODO: Zena: Move every enum variant handler to a separate function
fn handle_request(
    our: &Address,
    state: &mut State,
    body: &[u8],
    source: &Address,
) -> anyhow::Result<()> {
    match serde_json::from_slice::<TgRequest>(body)? {
        TgRequest::RegisterToken(token) => {
            state.tg_key = token.clone();
            state.api_url = format!("{}{}", BASE_API_URL, token.clone());
            state.current_offset = 0;
            state.api = Some(Api {
                api_url: state.api_url.clone(),
            });
            state.save();

            let update_params = get_updates_params(state.current_offset);
            request_no_wait(&state.api_url, "getUpdates", Some(update_params))?;
            send_response(TgResponse::RegisterToken(Ok(())))
        }
        TgRequest::Subscribe => {
            if !state.subscribers.contains(source) {
                state.subscribers.push(source.clone());
                state.save();
            }
            send_response(TgResponse::Subscribe(Ok(())))
        }
        TgRequest::Unsubscribe => {
            state.subscribers.retain(|x| x != source);
            state.save();
            send_response(TgResponse::Unsubscribe(Ok(())))
        }
        TgRequest::GetFile(file_id) => {
            let Some(ref api) = state.api else {
                return Err(anyhow::anyhow!("api not initialized"));
            };
            let get_file_params = GetFileParams {
                file_id,
            };

            let file_path = api
                .get_file(&get_file_params)?
                .result
                .file_path
                .ok_or_else(|| anyhow::anyhow!("file_path not found"))?;
            let download_url = format!(
                "https://api.telegram.org/file/bot{}/{}",
                state.tg_key.clone(),
                file_path
            );

            let outgoing_request = OutgoingHttpRequest {
                method: "GET".to_string(),
                version: None,
                url: download_url,
                headers: HashMap::new(),
            };
            let body_bytes = serde_json::to_vec(&HttpClientAction::Http(outgoing_request))?;
            let _ = Request::to(("our", "http_client", "distro", "sys"))
                .body(body_bytes)
                .send_and_await_response(30)??;
            let Some(blob) = get_blob() else {
                return Err(anyhow::anyhow!("blob not found"));
            };
            Response::new()
                .body(serde_json::to_vec(&TgResponse::GetFile(Ok(())))?)
                .blob(blob)
                .send()
                .map_err(|e| anyhow::anyhow!("Failed to send response: {}", e))
        }
        TgRequest::SendMessage(_) => Ok(()),
    }
}

call_init!(init);
fn init(our: Address) {
    let mut state = State::fetch();
    loop {
        match handle_message(&our, &mut state) {
            Ok(()) => {}
            Err(e) => {
                println!("tg: error: {:?}", e);
            }
        };
    }
}
