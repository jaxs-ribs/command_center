use crate::kinode::process::llm::{
    claude_chat, embedding, groq_chat, openai_chat, register_claude_api_key, register_groq_api_key,
    register_openai_api_key,
};
use crate::kinode::process::stt::{openai_transcribe, register_api_key as register_stt_key};
use crate::kinode::process::tg::{
    get_file, register_token, send_message, subscribe, unsubscribe, SendMessageParams, TgRequest,
};
use kinode_process_lib::{await_message, call_init, get_blob, println, Address, Message};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_request(body: &[u8]) -> anyhow::Result<(), String> {
    let Ok(TgRequest::SendMessage(message)) = serde_json::from_slice(body) else {
        return Err("unexpected response".to_string());
    };
    let mut text = message.text.clone();
    if let Some(voice) = message.voice.clone() {
        let _ = get_file(&voice.file_id);
        let Some(audio_blob) = get_blob() else {
            return Err("failed to get blob".to_string());
        };
        let transcript = openai_transcribe(&audio_blob.bytes)?;
        text += &transcript;
    }
    let params = SendMessageParams {
        chat_id: message.chat_id,
        text,
        voice: None,
    };
    send_message(&params);
    // TODO: 
    // let answer = get_groq_answer(&text)?;
    // let _message = send_bot_message(&answer, message.chat.id);
    Ok(())
}

fn handle_message(_our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;
    match std::str::from_utf8(&message.body()) {
        Ok(utf8_str) => println!("UTF-8 message content: {}", utf8_str),
        Err(e) => println!("Error converting message to UTF-8: {}", e),
    }
    match message {
        Message::Request { body, .. } => handle_request(&body).map_err(|e| anyhow::anyhow!(e)),
        Message::Response { .. } => Ok(()),
    }
}

call_init!(init);
fn init(our: Address) {
    let a = register_groq_api_key("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");
    let b = register_stt_key("sk-proj-J2y0MMBBYhLaw6iI680bT3BlbkFJPeH4fI3cumGNe6M6mbLX");
    let c = register_token("7327137177:AAHc5hXGmnUEI6CxrnlTYQTTGVG4Kphu288");
    let d = subscribe();

    println!("Groq API key registration result: {:?}", a);
    println!("STT API key registration result: {:?}", b);
    println!("Telegram token registration result: {:?}", c);
    println!("Subscription result: {:?}", d);

    loop {
        match handle_message(&our) {
            Ok(_) => {}
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
