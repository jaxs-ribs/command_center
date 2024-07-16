use kinode_process_lib::{await_message, call_init, println, Address, Message, Response};
use crate::kinode::process::stt::SttRequest;

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

call_init!(init);
fn init(_our: Address) {
    println!("begin");

    let a: SttRequest;
    // loop {
    //     match await_message() {
    //         Err(send_error) => println!("got SendError: {send_error}"),
    //         Ok(ref message) => match handle_message(message) {
    //             Ok(_) => {}
    //             Err(e) => println!("got error while handling message: {e:?}"),
    //         },
    //     }
    // }
}
