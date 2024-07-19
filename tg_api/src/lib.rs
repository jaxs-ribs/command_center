use crate::exports::kinode::process::tg::{Guest, SendMessageParams, TgRequest, TgResponse};
use kinode_process_lib::Request;

wit_bindgen::generate!({
    path: "target/wit",
    world: "tg-uncentered-dot-os-api-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn send_tg_request(request: TgRequest) -> anyhow::Result<TgResponse> {
    Request::new()
        .target(("our", "tg", "command_center", "uncentered.os"))
        .body(request)
        .send_and_await_response(5)??
        .body()
        .try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert response"))
}

fn handle_tg_response<T>(response: anyhow::Result<TgResponse>, expected: fn(TgResponse) -> Option<Result<T, String>>) -> Result<T, String> {
    match response {
        Ok(resp) => expected(resp).unwrap_or_else(|| Err("Unexpected response type".to_string())),
        Err(e) => Err(format!("{e:?}")),
    }
}

struct Api;
impl Guest for Api {
    fn register_token(token: String) -> Result<(), String> {
        handle_tg_response(send_tg_request(TgRequest::RegisterToken(token)), |r| {
            if let TgResponse::RegisterToken(result) = r { Some(result) } else { None }
        })
    }

    fn subscribe() -> Result<(), String> {
        handle_tg_response(send_tg_request(TgRequest::Subscribe), |r| {
            if let TgResponse::Subscribe(result) = r { Some(result) } else { None }
        })
    }

    fn unsubscribe() -> Result<(), String> {
        handle_tg_response(send_tg_request(TgRequest::Unsubscribe), |r| {
            if let TgResponse::Unsubscribe(result) = r { Some(result) } else { None }
        })
    }

    fn get_file(file_id: String) -> Result<(), String> {
        handle_tg_response(send_tg_request(TgRequest::GetFile(file_id)), |r| {
            if let TgResponse::GetFile(result) = r { Some(result) } else { None }
        })
    }

    fn send_message(params: SendMessageParams) -> Result<(), String> {
        handle_tg_response(send_tg_request(TgRequest::SendMessage(params)), |r| {
            if let TgResponse::SendMessage(result) = r { Some(result) } else { None }
        })
    }
}
export!(Api);