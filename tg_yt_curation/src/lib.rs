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
use helpers::{
    extract_youtube_video_id,
    parse_start_command,
    parse_register_command,
    parse_six_digit_number,
    is_curation_message,
    hash_youtube_curation,
    create_youtube_embed_src,
    create_post_entry,
    create_set_post_request,
};

mod lm;
use lm::use_groq;

const TG_TOKEN: &str = include_str!("../../TG_TOKEN");

wit_bindgen::generate!({
    path: "target/wit",
    world: "command-center-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(
    message: &Message,
    state: &mut State,
) -> anyhow::Result<()> {
    if !message.is_request() {
        return Err(anyhow::anyhow!("unexpected Response: {:?}", message));
    }

    let Ok(TgRequest::SendMessage(tg_message)) = serde_json::from_slice(message.body()) else {
        return Err(anyhow::anyhow!("unexpected request"));
    };

    let chat_id = tg_message.chat_id.to_string();
    let command = TGMessage::try_from(tg_message.text.as_str())?;
    let response_text = match command {
        TGMessage::Start() => handle_start(),
        TGMessage::Register(address) => handle_register(&chat_id, &address, state),
        TGMessage::Authenticate(code) => handle_authenticate(&chat_id, code, state),
        TGMessage::CurationMSGToEmbedLinkRequest(msg) => handle_curate_youtube(&chat_id, msg, state),
        _ => format!("I don't understand that command: \"{}\"", tg_message.text),
    };

    // send response from commands above to user 
    let _ = send_message(&SendMessageParams {
        chat_id: tg_message.chat_id,
        text: response_text,
        voice: None,
    });
    state.save()?;
    Ok(())
}


fn handle_start() -> String {
    "Please start the registration process by entering: \n\n'/register your_node.os'\n\n".to_string()
}

fn handle_register(chat_id: &str, address: &str, state: &mut State) -> String {
    println!("TG YT Curator: Registering address: {} for chat_id: {}", address, chat_id);

    //let code: u64 = 666666;
    let code: u64 = rand::thread_rng().gen_range(100_000..1_000_000);
    println!("TG YT Curator: Code is {}", code);
    
    match send_code_to_terminal(address, &code) {
        Ok(_) => {
            state.pending_codes.insert(chat_id.to_string(), (address.to_string(), code));
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

fn handle_authenticate(chat_id: &str, code: u64, state: &mut State) -> String {
    println!("TG YT Curator: Received auth code: {} for chat_id: {}", code, chat_id);
    if let Some((kinode_address, expected_code)) = state.pending_codes.get(chat_id) {
        if code == *expected_code {
            state.address_book.insert(chat_id.to_string(), kinode_address.clone());
            state.pending_codes.remove(&chat_id.to_string());
            "Authentication successful. You are now registered.".to_string()
        } else {
            "Incorrect code. Please try again.".to_string()
        }
    } else {
        "No pending registration found. Please start with the register command.".to_string()
    }
}

fn handle_curate_youtube(
    chat_id: &str, 
    msg: String, 
    state: &mut State
) -> String {
    if !state.address_book.contains_key(chat_id) {
        return "You are not registered. Please register first..".to_string();
    }

    let kinode_address = state.address_book.get(chat_id).unwrap();

    match curation_msg_to_youtube_curation(&msg) {
        Ok((youtube_curation, combined_uuid)) => {

            let set_post_request = create_set_post_request(&combined_uuid, kinode_address, &youtube_curation.post_entry);

            let address: (&str, &str, &str, &str) = (kinode_address.as_str(), "hq", "hq", "uncentered.os");
            let request_body = serde_json::to_vec(&set_post_request).unwrap();

            println!("TG YT Curator: Address: {:?}", address);
            println!("TG YT Curator: set_post_request: {:?}", set_post_request);
            println!("TG YT Curator: request_body: {:?}", request_body);

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

fn curation_msg_to_youtube_curation(telegram_msg: &str) -> anyhow::Result<(YoutubeCuration, String)> {
    println!("TG YT Curator: Telegram msg: {:?}", telegram_msg);

    let struct_from_lm: TGYoutubeCurationMessage = use_groq(telegram_msg)?;
    println!("TG YT Curator: Youtube curation message: {:?}", struct_from_lm);

    //let start_time = struct_from_lm.start_time.map(|s| s.parse::<u64>().unwrap_or(0));
    let start_time = struct_from_lm.start_time.unwrap_or_else(|| "0".to_string());
    let duration = struct_from_lm.duration.unwrap_or_else(|| "30".to_string());
    let end_time = (start_time.parse::<u64>().unwrap_or(0) + duration.parse::<u64>().unwrap_or(30)).to_string();

    let embed_params = YoutubeEmbedParams {
        video_id: extract_youtube_video_id(&struct_from_lm.share_link).unwrap_or_default().to_string(),
        start_time: Some(start_time),
        end_time: Some(end_time),
    };

    let post_entry = create_post_entry(&embed_params);
    println!("TG YT Curator: Youtube embed params: {:?}", embed_params);

    let embed_src = create_youtube_embed_src(&embed_params);
    println!("TG YT Curator: Youtube embed src: {:?}", embed_src);

    let curation_quote = struct_from_lm.curation_quote.filter(|q| !q.trim().is_empty());

    let combined_uuid = hash_youtube_curation(&embed_params);

    let youtube_curation = YoutubeCuration { embed_src, curation_quote, post_entry};


    Ok((youtube_curation, combined_uuid))
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
        println!("TG YT Curator state: {:#?}", state);
        //state.clear();
        //println!("TG YT Curator state after clear: {:#?}", state);
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(message, &mut state) {
                Ok(_) => {}
                Err(e) => println!("got error while handling message: {e:?}"),
            }
        }
    }
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
