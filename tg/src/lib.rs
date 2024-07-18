use kinode_process_lib::{
    await_message, call_init, get_blob,
    http::{self, HttpClientAction, OutgoingHttpRequest},
    println, Address, Message, Request, Response,
};
use crate::kinode::process::tg::{TgRequest, TgResponse};

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
    match message {
        Message::Request {
            ref body, source, ..
        } => handle_request(our, state, body, &source),
        Message::Response { .. } => Ok(()),
    }
}

fn handle_request(
    our: &Address,
    state: &mut State,
    body: &[u8],
    source: &Address,
) -> anyhow::Result<()> {
    match serde_json::from_slice::<TgRequest>(body)? {
        TgRequest::RegisterToken(_) => todo!(),
        TgRequest::Subscribe => todo!(),
        TgRequest::Unsubsribe => todo!(),
        TgRequest::GetFile(_) => todo!(),
        TgRequest::SendMessage(_) => todo!(),
    }
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
