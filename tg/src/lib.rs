use crate::kinode::process::tg::{
    SendMessageParams as WitSendMessageParams, TgRequest, TgResponse, Voice as WitVoice
};
use frankenstein::GetFileParams;
use frankenstein::MethodResponse;
use frankenstein::Update;
use frankenstein::UpdateContent;
use frankenstein::{SendMessageParams, TelegramApi};
use kinode_process_lib::{
    await_message, call_init, get_blob,
    http::client::{HttpClientAction, OutgoingHttpRequest},
    Address, LazyLoadBlob, Message, Request, Response,
};
use std::collections::HashMap;

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

fn send_response_with_blob(response: TgResponse, blob: LazyLoadBlob) -> anyhow::Result<()> {
    Response::new()
        .body(serde_json::to_vec(&response)?)
        .blob(blob)
        .send()
        .map_err(|e| anyhow::anyhow!("Failed to send response with blob: {}", e))
}

fn handle_message(state: &mut State) -> anyhow::Result<()> {
    let message = await_message()?;
    match message {
        Message::Request {
            ref body, source, ..
        } => handle_request(state, body, &source),
        Message::Response { source, .. } => {
            if !["http_server:distro:sys", "http_client:distro:sys"]
                .contains(&source.process.to_string().as_str())
            {
                return Err(anyhow::anyhow!("invalid source"));
            }
            handle_http_response(state)
        }
    }
}

fn handle_http_response(state: &mut State) -> anyhow::Result<()> {
    let Some(blob) = get_blob() else {
        return Err(anyhow::anyhow!("blob not found in http response"));
    };
    let Ok(response) = serde_json::from_slice::<MethodResponse<Vec<Update>>>(&blob.bytes) else {
        return Err(anyhow::anyhow!(
            "couldn't parse http response into a telegram update"
        ));
    };
    let Some(update) = response.result.get(0) else {
        return Ok(());
    };
    let UpdateContent::Message(msg) = &update.content else {
        return Err(anyhow::anyhow!("not a message"));
    };
    let voice = if let Some(voice_) = &msg.voice {
        Some(WitVoice {
            file_id: voice_.file_id.clone(),
            file_unique_id: voice_.file_unique_id.clone(),
            duration: voice_.duration.clone(),
            mime_type: voice_.mime_type.clone(),
            file_size: voice_.file_size.clone(),
        })
    } else {
        None
    };
    let wit_msg = WitSendMessageParams {
        chat_id: msg.chat.id as i64,
        text: msg.text.clone().unwrap_or_default(),
        voice,
    };
    let body = serde_json::to_vec(&TgRequest::SendMessage(wit_msg))?;

    for sub in state.subscribers.iter() {
        let _ = Request::new().target(sub.clone()).body(body.clone()).send();
    }

    // set current_offset based on the response, keep same if no updates
    let next_offset = response
        .result
        .last()
        .map(|u| u.update_id + 1)
        .unwrap_or(state.current_offset);
    state.current_offset = next_offset;
    let updates_params = frankenstein::GetUpdatesParams {
        offset: Some(state.current_offset as i64),
        limit: None,
        timeout: Some(15),
        allowed_updates: None,
    };

    request_no_wait(&state.api_url, "getUpdates", Some(updates_params))?;

    Ok(())
}

fn handle_request(state: &mut State, body: &[u8], source: &Address) -> anyhow::Result<()> {
    match serde_json::from_slice::<TgRequest>(body)? {
        TgRequest::RegisterToken(token) => handle_register_token(state, token),
        TgRequest::Subscribe => handle_subscribe(state, source),
        TgRequest::Unsubscribe => handle_unsubscribe(state, source),
        TgRequest::GetFile(file_id) => handle_get_file(state, file_id),
        TgRequest::SendMessage(params) => handle_send_message(state, params),
    }
}

fn handle_register_token(state: &mut State, token: String) -> anyhow::Result<()> {
    state.tg_key = token.clone();
    state.api_url = format!("{}{}", BASE_API_URL, token);
    // Only reset the current offset if the api is not initialized
    state.current_offset = if state.api.is_some() {
        state.current_offset
    } else {
        0
    };
    state.api = Some(Api {
        api_url: state.api_url.clone(),
    });
    state.save();

    let update_params = get_updates_params(state.current_offset);
    request_no_wait(&state.api_url, "getUpdates", Some(update_params))?;
    send_response(TgResponse::RegisterToken(Ok(())))
}

fn handle_subscribe(state: &mut State, source: &Address) -> anyhow::Result<()> {
    if !state.subscribers.contains(source) {
        state.subscribers.push(source.clone());
        state.save();
    }
    send_response(TgResponse::Subscribe(Ok(())))
}

fn handle_unsubscribe(state: &mut State, source: &Address) -> anyhow::Result<()> {
    state.subscribers.retain(|x| x != source);
    state.save();
    send_response(TgResponse::Unsubscribe(Ok(())))
}

fn handle_get_file(state: &State, file_id: String) -> anyhow::Result<()> {
    let Some(ref api) = state.api else {
        return Err(anyhow::anyhow!("api not initialized"));
    };
    let get_file_params = GetFileParams { file_id };

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
    send_response_with_blob(TgResponse::GetFile(Ok(())), blob)
}

fn handle_send_message(state: &State, params: WitSendMessageParams) -> anyhow::Result<()> {
    let Some(ref api) = state.api else {
        return Err(anyhow::anyhow!("api not initialized"));
    };
    let frankenstein_params = SendMessageParams::builder()
        .chat_id(frankenstein::ChatId::Integer(params.chat_id.into()))
        .text(params.text)
        .build();
    let _message = api.send_message(&frankenstein_params)?;
    send_response(TgResponse::SendMessage(Ok(())))
}

call_init!(init);
fn init(_: Address) {
    let mut state = State::fetch();
    loop {
        match handle_message(&mut state) {
            Ok(()) => {}
            Err(e) => {
                println!("tg: error: {:?}", e);
            }
        };
    }
}
