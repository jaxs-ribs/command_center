use crate::kinode::process::tg::{
    register_token, send_message, SendMessageParams, TgRequest,
};
use kinode_process_lib::{
    await_message, call_init, get_typed_state, println, Address, Message, Request,
};

mod helpers;
mod structs;

use helpers::*;
use structs::*;

const TG_TOKEN: &str = include_str!("../../TG_TOKEN");

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

pub fn send_code_to_terminal(node: &str, code: &str) -> anyhow::Result<()> {
    let address: (&str, &str, &str, &str) = (node, "hi", "terminal", "sys");
    // TODO: Zena: Test this
    Ok(Request::to(address)
        .body(serde_json::to_vec(&code)?)
        .send()?)
}

fn handle_request(state: &mut State, body: &[u8]) -> anyhow::Result<()> {
    let Ok(TgRequest::SendMessage(message)) = serde_json::from_slice(body) else {
        return Err(anyhow::anyhow!("unexpected request"));
    };
    let response_text;

    let chat_id = message.chat_id;
    let text = message.text.clone();

    if let Some(kinode_address) = parse_register_command(&text) {
        let code: u64 = rand::random::<u64>() % 1_000_000;
        let code_string = format!("{:06}", code);

        match send_code_to_terminal(&kinode_address, &code_string) {
            Ok(_) => {
                state.pending_codes.insert(chat_id, (kinode_address, code));
                response_text = "Code sent, check your terminal".to_string();
            }
            Err(_) => {
                response_text =
                    "Failed to send code, give valid address and make sure your node is up"
                        .to_string();
            }
        }
    // TODO: Zena: Clear the code after 3 false tries for security reasons
    } else if let Some(code) = parse_six_digit_number(&text) {
        if let Some((kinode_address, expected_code)) = state.pending_codes.get(&chat_id) {
            if code == *expected_code {
                state.address_book.insert(chat_id, kinode_address.clone());
                response_text = "Registered successfully".to_string();
            } else {
                response_text = "Wrong code, try again".to_string();
            }
        } else {
            response_text = "No code pending, send register command first".to_string();
        }
    } else if let Some(post_id) = parse_twitter_link(&text) {
        if let Some(kinode_address) = state.address_book.get(&chat_id) {
            // TODO: Zena: Test this
            let address: (&str, &str, &str, &str) = (kinode_address, "hq", "hq", "uncentered.os");
            // TODO: Zena: Streams need to somehow be dynamic
            let request = TgCuratorRequest::CurateLink {
                stream_name: "default".to_string(),
                site: "x".to_string(),
                post_id,
            };
            let request_body = serde_json::to_vec(&request)?;
            match Request::to(address)
                .body(request_body)
                .send_and_await_response(5)
            {
                Ok(_) => response_text = "Link curated successfully".to_string(),
                Err(e) => response_text = format!("Failed to curate link: {}", e),
            }
        } else {
            response_text = "You are not registered, send register command first".to_string();
        }
    } else {
        response_text = "Send /register <kinode_address> to register, a code to confirm registration, or a Twitter link to curate.".to_string();
    }

    let _ = send_message(&SendMessageParams {
        chat_id: message.chat_id,
        text: response_text,
        voice: None,
    });
    Ok(())
}

fn handle_message(state: &mut State, _our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;
    if let Message::Request { body, .. } = message {
        handle_request(state, &body).map_err(anyhow::Error::msg)?;
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    let mut state = load_state();
    // TODO: Zena: register token
    register_token(TG_TOKEN).expect("Failed to register token");


    loop {
        if let Err(e) = handle_message(&mut state, &our) {
            println!("Error: {:?}", e);
        }
    }
}
