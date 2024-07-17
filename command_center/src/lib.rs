use crate::kinode::process::stt::{SttRequest, SttResponse, register_api_key, openai_transcribe};
use crate::kinode::process::llm::{LlmRequest, LlmResponse};
use kinode_process_lib::{await_message, call_init, println, Address, Message, Response, Request};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});


fn stt() -> anyhow::Result<()> {
    let result = register_api_key("u no taek key");
    println!("Result is going to be {:?}", result);

    let audio = vec![0; 1024];
    let result = openai_transcribe(&audio);
    println!("Result is going to be {:?}", result);

    Ok(())
}

fn llm() -> anyhow::Result<()> {
    // register openai
    println!("Starting");
    let LlmResponse::RegisterOpenaiApiKey(result) = Request::new()
        .target(("our", "llm", "command_center", "uncentered.os"))
        .body(LlmRequest::RegisterOpenaiApiKey("sk-proj-J2y0MMBBYhLaw6iI680bT3BlbkFJPeH4fI3cumGNe6M6mbLX".to_string()))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        return Err(anyhow::anyhow!("Failed to register OpenAI API key"));
    };
    println!("result: {:?}", result);

    // register groq

    // register claude

    // Call the chat method for openai

    // Call the embedding method for openai

    // Call the chat method for claude

    // Call the chat method for groq

    Ok(())
}

fn test() -> anyhow::Result<()> {
    // stt()?;
    llm()?;
    Ok(())
}

call_init!(init);
fn init(_our: Address) {
    println!("begin");
    match test() {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error: {e}"),
    }
}