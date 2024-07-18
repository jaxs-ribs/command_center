use kinode_process_lib::{
    await_message, call_init, get_blob, println, Address, Message, Request,
    Response, http::{self, HttpClientAction, OutgoingHttpRequest},
};

wit_bindgen::generate!({
    path: "target/wit",
    world: "tg-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

call_init!(init);
fn init(_: Address) {
    // loop {
    //     match handle_message(&mut state) {
    //         Ok(_) => {}
    //         Err(e) => println!("got error while handling message: {e:?}"),
    //     }
    // }
}
