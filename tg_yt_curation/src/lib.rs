use std::convert::TryFrom;
use rand::Rng;

use crate::kinode::process::tg::{
    subscribe, register_token,send_message,
    SendMessageParams, TgRequest
};
use kinode_process_lib::{await_message, call_init, println, Address, Message, Request};

mod structs;
use structs::{State,TGMessage, TGYoutubeCurationMessage,YoutubeCuration,YoutubeEmbedParams,
    load_state,
};

mod helpers;
use helpers::{parse_start_command,parse_register_command,parse_six_digit_number,is_curation_message};

mod lm;
use lm::use_groq;

const TG_TOKEN: &str = include_str!("../../TG_TOKEN");
//const GROQ_API_KEY: &str = include_str!("../../GROQ_API_KEY");

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});


fn handle_message(
    our: &Address,
    message: &Message,
    state: &mut State,
) -> anyhow::Result<()> {
    if !message.is_request() {
        return Err(anyhow::anyhow!("unexpected Response: {:?}", message));
    }

    let Ok(TgRequest::SendMessage(tg_message)) = serde_json::from_slice(message.body()) else {
        return Err(anyhow::anyhow!("unexpected request"));
    };

    let command = TGMessage::try_from(tg_message.text.as_str())?;
    let response_text = match command {
        TGMessage::Start() => handle_start(),
        TGMessage::Register(address) => handle_register(our, &address, state),
        TGMessage::Authenticate(code) => handle_authenticate(our, code, state),
        TGMessage::CurationMSGToEmbedLinkRequest(msg) => handle_curate_youtube(our, msg, state),
        _ => "I don't understand that command.".to_string(),
    };

    let _ = send_message(&SendMessageParams {
        chat_id: tg_message.chat_id,
        text: response_text,
        voice: None,
    });
    state.save()?;
    Ok(())
}

impl TryFrom<&str> for TGMessage {
    type Error = anyhow::Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        if let Some(_) = parse_start_command(text) {
            Ok(TGMessage::Start())
        } else if let Some(address) = parse_register_command(text) {
            Ok(TGMessage::Register(address))
        } else if let Some(code) = parse_six_digit_number(text) {
            Ok(TGMessage::Authenticate(code))
        } else if is_curation_message(text) {
           Ok(TGMessage::CurationMSGToEmbedLinkRequest(text.to_string())) 
        } else {
            Ok(TGMessage::Unknown(text.to_string()))
        }
    }
}

fn handle_start() -> String {
    "Welcome to the TG YT Curator!\nPlease start the registration process by entering: \n\n'/register your_node.os'\n\n".to_string()
}

fn handle_register(our: &Address, address: &str, state: &mut State) -> String {
    println!("TG YT Curator: Registering address: {}", address);
    //let code: u64 = 666666;
    let code: u64 = rand::thread_rng().gen_range(0..1_000_000);
    println!("TG YT Curator: Code is {}", code);
    
    match send_code_to_terminal(address, &code) {
        Ok(_) => {
            state.pending_codes.insert(our.node.clone(), (address.to_string(), code));
            "Registration code sent to your terminal. Please enter the code to complete registration.".to_string()
        },
        Err(_) => "Failed to send registration code. Please ensure your Kinode address is correct and your node is online.".to_string(),
    }
}

fn send_code_to_terminal(kinode_address: &str, code: &u64) -> anyhow::Result<()> {
    let address: (&str, &str, &str, &str) = (kinode_address, "hi", "terminal", "sys");
    Ok(Request::to(address)
        .body(serde_json::to_vec(&code)?)
        .send()?)
}

fn handle_authenticate(our: &Address, code: u64, state: &mut State) -> String {
    println!("TG YT Curator: Received auth code: {}", code);
    if let Some((kinode_address, expected_code)) = state.pending_codes.get(&our.node) {
        if code == *expected_code {
            state.address_book.insert(our.node.clone(), kinode_address.clone());
            state.pending_codes.remove(&our.node);
            "Authentication successful. You are now registered.".to_string()
        } else {
            "Incorrect code. Please try again.".to_string()
        }
    } else {
        "No pending registration found. Please start with the register command.".to_string()
    }
}

fn handle_curate_youtube(our: &Address, msg: String, state: &mut State) -> String {
    if !state.address_book.contains_key(&our.node) {
        return "You are not registered. Please register first, boi.".to_string();
    }

    match curation_msg_to_embed_link(&msg) {
        Ok(youtube_curation) => {
            let address: (&str, &str, &str, &str) = (our.node.as_str(), "hq", "hq", "uncentered.os");
            let request_body = serde_json::to_vec(&youtube_curation).unwrap();

            println!("TG YT Curator: Request body: {:?}", request_body);

            match Request::to(address)
                .body(request_body)
                .send_and_await_response(5)
            {
                Ok(_) => "YouTube link curated successfully.".to_string(),
                Err(e) => format!("Failed to curate YouTube link: {}", e),
            }
        },
        Err(e) => format!("Failed to process YouTube curation: {}", e),
    }
}

fn curation_msg_to_embed_link(telegram_msg: &str) -> anyhow::Result<YoutubeCuration> {
    println!("TG YT Curator: Telegram msg: {:?}", telegram_msg);

    let struct_from_lm: TGYoutubeCurationMessage = use_groq(telegram_msg)?;
    println!("TG YT Curator: Youtube curation message: {:?}", struct_from_lm);

    let start_time = struct_from_lm.start_time.unwrap_or_default().parse::<u64>().unwrap_or(0);
    let duration = struct_from_lm.duration.unwrap_or_else(|| "30".to_string()).parse::<u64>().unwrap_or(30);
    let end_time = start_time + duration;

    let embed_params = YoutubeEmbedParams {
        video_id: struct_from_lm.share_link.split("v=").nth(1).unwrap_or_default().to_string(),
        start_time: start_time.to_string(),
        end_time: end_time.to_string(),
    };
    println!("TG YT Curator: Youtube embed params: {:?}", embed_params);

    Ok(YoutubeCuration { embed_params, curation_quote: struct_from_lm.curation_quote })
}

call_init!(init);
fn init(our: Address) {
    let mut state = load_state();
    match register_token(TG_TOKEN) {
        Ok(_) => println!("TG YT Curator: Token registered"),
        Err(e) => println!("TG YT Curator: Failed to register token: {}", e),
    }

    let _ = subscribe();

    loop {
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message, &mut state) {
                Ok(_) => {}
                Err(e) => println!("got error while handling message: {e:?}"),
            }
        }
    }
}
