use crate::kinode::process::tg::{
    register_token, send_message, SendMessageParams, TgRequest,
};
use kinode::process::tg::subscribe;
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
    Ok(Request::to(address)
        .body(serde_json::to_vec(&code)?)
        .send()?)
}

fn handle_request(state: &mut State, body: &[u8]) -> anyhow::Result<()> {
    println!("TG Curator: Handling request");
    let Ok(TgRequest::SendMessage(message)) = serde_json::from_slice(body) else {
        return Err(anyhow::anyhow!("unexpected request"));
    };
    println!("TG Curator: Message is {:?}", message);
    let response_text;

    let chat_id = message.chat_id;
    let text = message.text.clone();

    if let Some(kinode_address) = parse_register_command(&text) {
        println!("TG Curator: Sending code to terminal for {}", kinode_address);
        let code: u64 = rand::random::<u64>() % 1_000_000;
        let code_string = format!("{:06}", code);
        println!("TG Curator: Code is {}", code_string);

        match send_code_to_terminal(&kinode_address, &code_string) {
            Ok(_) => {
                println!("TG Curator: Code sent, check your terminal");
                state.pending_codes.insert(chat_id, (kinode_address, code));
                response_text = "Code sent, check your terminal".to_string();
            }
            Err(_) => {
                println!("TG Curator: Failed to send code, give valid address and make sure your node is up");
                response_text =
                    "Failed to send code, give valid address and make sure your node is up"
                        .to_string();
            }
        }
    // TODO: Zena: Clear the code after 3 false tries for security reasons
    } else if let Some(code) = parse_six_digit_number(&text) {
        println!("TG Curator: Code is {}", code);
        if let Some((kinode_address, expected_code)) = state.pending_codes.get(&chat_id) {
            println!("TG Curator: Expected code is {}", expected_code);
            if code == *expected_code {
                println!("TG Curator: Code is correct");
                state.address_book.insert(chat_id, kinode_address.clone());
                response_text = "Registered successfully".to_string();
            } else {
                println!("TG Curator: Code is wrong");
                response_text = "Wrong code, try again".to_string();
            }
        } else {
            println!("TG Curator: No code pending");
            response_text = "No code pending, send register command first".to_string();
        }
    } else if let Some(post_id) = parse_twitter_link(&text) {
        println!("TG Curator: Post ID is {:?}", post_id);
        if let Some(kinode_address) = state.address_book.get(&chat_id) {
            println!("TG Curator: Kinode address is {:?}", kinode_address);
            // TODO: Zena: Test this
            let address: (&str, &str, &str, &str) = (kinode_address, "hq", "hq", "uncentered.os");
            println!("TG Curator: Sending request to {:?}", address);
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
            println!("TG Curator: You are not registered, send register command first");
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
    state.save()?;
    Ok(())
}

fn handle_message(state: &mut State, _our: &Address) -> anyhow::Result<()> {
    println!("TG Curator: Handling message");
    let message = await_message()?;
    if let Message::Request { body, .. } = message {
        handle_request(state, &body).map_err(anyhow::Error::msg)?;
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("TG Curator initialized");
    let mut state = load_state();
    match register_token(TG_TOKEN) {
        Ok(_) => println!("TG Curator: Token registered"),
        Err(e) => println!("TG Curator: Failed to register token: {}", e),
    }
    let _ = subscribe();

    loop {
        if let Err(e) = handle_message(&mut state, &our) {
            println!("Error: {:?}", e);
        }
    }
}
