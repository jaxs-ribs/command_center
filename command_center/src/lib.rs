use crate::kinode::process::stt::{SttRequest, SttResponse, register_api_key, openai_transcribe};
use kinode_process_lib::{await_message, call_init, println, Address, Message, Response, Request};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});


fn test() -> anyhow::Result<()> {
    let result = register_api_key("u no taek key");
    println!("Result is going to be {:?}", result);

    let audio = vec![0; 1024];
    let result = openai_transcribe(&audio);
    println!("Result is going to be {:?}", result);

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