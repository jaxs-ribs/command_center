use crate::exports::kinode::process::stt::{Guest, SttRequest, SttResponse};
use kinode_process_lib::{vfs, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "stt-uncentered-dot-os-api-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

// TODO: Zena: Should we maybe activate inherit(true) and explicitly send a response on fail?
fn register_api_key(key: String) -> anyhow::Result<Result<String, String>> {
    let SttResponse::RegisterApiKey(result) = Request::new()
        .target(("our", "stt", "command_center", "uncentered.os"))
        .body(SttRequest::RegisterApiKey(key))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register API key"));
    };
    Ok(result)
}

fn openai_transcribe(audio: Vec<u8>) -> anyhow::Result<Result<String, String>> {
    let SttResponse::OpenaiTranscribe(result) = Request::new()
        .target(("our", "stt", "command_center", "uncentered.os"))
        .body(SttRequest::OpenaiTranscribe(audio))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to transcribe audio"));
    };
    Ok(result)
}

struct Api;
impl Guest for Api {
    fn register_api_key(key: String) -> Result<String, String> {
        match register_api_key(key) {
            Ok(result) => result,
            Err(e) => Err(format!("{e:?}")),
        }
    }

    fn openai_transcribe(audio: Vec<u8>) -> Result<String, String> {
        match openai_transcribe(audio) {
            Ok(result) => result,
            Err(e) => Err(format!("{e:?}")),
        }
    }
}
export!(Api);
