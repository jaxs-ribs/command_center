use crate::kinode::process::{
    // llm::{groq_chat, register_groq_api_key},
    // stt::{openai_transcribe, register_api_key as register_stt_key},
    tg::{register_token, subscribe},
};
use kinode_process_lib::{await_message, call_init, println, Address, Message};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_request(_body: &[u8]) -> Result<(), String> {
    // let message = serde_json::from_slice::<TgRequest>(body)
    //     .map_err(|e| format!("Failed to parse TgRequest: {}", e))?;

    // let TgRequest::SendMessage(message) = message else {
    //     return Err("Unexpected request type".to_string());
    // };

    // let mut text = message.text.clone();
    // if let Some(voice) = message.voice {
    //     get_file(&voice.file_id).map_err(|e| format!("Failed to get file: {}", e))?;
    //     let audio_blob = get_blob().ok_or("Failed to get blob")?;
    //     text += &openai_transcribe(&audio_blob.bytes)
    //         .map_err(|e| format!("Transcription failed: {}", e))?;
    // }

    // let answer = groq_chat(&text, None)
    //     .map_err(|e| format!("Groq chat failed: {}", e))?;

    // send_message(&SendMessageParams {
    //     chat_id: message.chat_id,
    //     text: answer,
    //     voice: None,
    // }).map_err(|e| format!("Failed to send message: {}", e))?;

    Ok(())
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
    let results = [
        //("Groq API key", register_groq_api_key("<KEY>")),
        //("STT API key", register_stt_key("<KEY>")),
        ("Telegram token", register_token("<KEY>").map(|_| "Success".to_string())),
        ("Subscription", subscribe().map(|_| "Success".to_string())),
    ];

    for (name, result) in results {
        match result {
            Ok(msg) => println!("{} registration result: Success - {}", name, msg),
            Err(e) => println!("{} registration failed: {:?}", name, e),
        }
    }

    loop {
        if let Err(e) = handle_message(&our) {
            println!("Error: {:?}", e);
        }
    }
}
