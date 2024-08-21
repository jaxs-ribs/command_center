use crate::kinode::process::{
    llm::{groq_chat, register_groq_api_key},
    stt::{openai_transcribe, register_api_key as register_stt_key},
    tg::{get_file, register_token, send_message, subscribe, SendMessageParams, TgRequest},
};
use kinode_process_lib::{await_message, call_init, get_blob, println, Address, Message};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_request(body: &[u8]) -> Result<(), String> {
    let Ok(TgRequest::SendMessage(message)) = serde_json::from_slice(body) else {
        return Err("unexpected response".to_string());
    };
    let mut text = message.text.clone();
    if let Some(voice) = message.voice {
        get_file(&voice.file_id).map_err(|_| "failed to get file")?;
        let audio_blob = get_blob().ok_or("failed to get blob")?;
        text += &openai_transcribe(&audio_blob.bytes)?;
    }
    let answer = groq_chat(&text, None)?;
    send_message(&SendMessageParams {
        chat_id: message.chat_id,
        text: answer,
        voice: None,
    })
}

fn handle_message(_our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;
    if let Message::Request { body, .. } = message {
        handle_request(&body).map_err(anyhow::Error::msg)?;
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("Starting command center");
    let results = [
        ("Groq API key", register_groq_api_key("<KEY>")),
        ("STT API key", register_stt_key("<KEY>")),
        ("Telegram token", register_token("<KEY>").map(|_| "Success".to_string())),
        ("Subscription", subscribe().map(|_| "Success".to_string())),
    ];

    for (name, result) in results {
        println!("{} registration result: {:?}", name, result);
    }

    loop {
        if let Err(e) = handle_message(&our) {
            println!("Error: {:?}", e);
        }
    }
}
