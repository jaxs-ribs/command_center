use crate::kinode::process::llm::{
    claude_chat, embedding, groq_chat, openai_chat, register_claude_api_key, register_groq_api_key,
    register_openai_api_key,
};
use crate::kinode::process::stt::{openai_transcribe, register_api_key};
use kinode_process_lib::{call_init, println, Address};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn _stt() -> anyhow::Result<()> {
    let result = register_api_key("u no taek key");
    println!("Result is going to be {:?}", result);

    let audio = vec![0; 1024];
    let result = openai_transcribe(&audio);
    println!("Result is going to be {:?}", result);

    Ok(())
}

// Update the llm() function to demonstrate the use of optional model parameters
fn llm() -> anyhow::Result<()> {
    println!("Starting LLM operations");

    // Register API keys
    let api_keys = [
        ("OpenAI", register_openai_api_key("sk-proj-J2y0MMBBYhLaw6iI680bT3BlbkFJPeH4fI3cumGNe6M6mbLX")),
        ("Groq", register_groq_api_key("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")),
        ("Claude", register_claude_api_key("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")),
    ];

    for (provider, result) in api_keys {
        println!("{} API key registered: {:?}", provider, result);
    }

    // Get embedding
    let embedding_result = embedding("This is an embedding test", None);
    println!("Embedding result length: {:?}", embedding_result);

    // Chat with different models
    let chat_queries = [
        ("OpenAI (default)", openai_chat("What is the capital of France?", None)),
        ("OpenAI (GPT-4)", openai_chat("What is the capital of France?", Some("gpt-4"))),
        ("Groq (default)", groq_chat("What is the capital of Germany?", None)),
        ("Claude (default)", claude_chat("What is the capital of Japan?", None)),
    ];

    for (model, result) in chat_queries {
        println!("{} chat result: {:?}", model, result);
    }

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
