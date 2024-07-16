use crate::kinode::process::stt::{SttRequest, SttResponse};
use kinode_process_lib::{await_message, call_init, println, Address, Message, Response, Request};

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});


fn test() -> anyhow::Result<()> {
    let SttResponse::RegisterApiKey(result) = Request::new()
        .target(("our", "stt", "command_center", "uncentered.os"))
        .body(SttRequest::RegisterApiKey("test".to_string()))
        .send_and_await_response(5)??
        .body()
        .try_into()?
    else {
        println!("IT'S PANIC TIME KYLE");
        return Err(anyhow::anyhow!("IT'S PANIC TIME KYLE"));
    };

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