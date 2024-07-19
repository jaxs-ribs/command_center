use crate::exports::kinode::process::tg::{Guest, TgRequest, TgResponse, SendMessageParams};
use kinode_process_lib::Request;

wit_bindgen::generate!({
    path: "target/wit",
    world: "tg-uncentered-dot-os-api-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});


struct Api;
impl Guest for Api {
    fn register_token(token: String) -> Result<(), String> {
        todo!()
    }

    fn subscribe() -> Result<(), String> {
        todo!()
    }

    fn unsubscribe() -> Result<(), String> {
        todo!()
    }

    fn get_file(file_id: String) -> Result<(), String> {
        todo!()
    }

    fn send_message(params: SendMessageParams) -> Result<(), String> {
        todo!()
    }
}
export!(Api);
