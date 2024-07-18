use kinode_process_lib::{
    await_message, call_init, get_blob,
    http::{self, HttpClientAction, OutgoingHttpRequest},
    println, Address, Message, Request, Response,
};

mod state;
use state::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "tg-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<()> {
    let message = await_message()?;



    Ok(())
}

call_init!(init);
fn init(our: Address) {
    let mut state = State::fetch();
    loop {
        match handle_message(&our, &mut state) {
            Ok(()) => {}
            Err(e) => {
                println!("tg: error: {:?}", e);
            }
        };
    }
}
