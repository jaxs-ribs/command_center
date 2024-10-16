use crate::structs::TGYoutubeCurationMessage;

pub fn use_groq(_msg: &str) -> anyhow::Result<TGYoutubeCurationMessage> {
    // Return a fake TGYoutubeCurationMessage
    Ok(TGYoutubeCurationMessage {
        share_link: "https://youtu.be/dQw4w9WgXcQ?t=0".to_string(),
        start_time: Some("0".to_string()),
        duration: Some("30".to_string()),
        curation_quote: Some("Never gonna give you up!".to_string()),
    })
}

// ... rest of the existing code ...

////how should I design this llm instruction for getting the youtube embed params?
//use serde_json::json;
//
//const GROQ_API_KEY: &str = include_str!("../../GROQ_API_KEY");
//const example_user_tg_message: &str = "https://youtu.be/4ol3dDzgHrs?t=2&si=tAlasldCadj\n\nduration 30s\n\nhow will they be able to keep this streak up? Go cubs!";
//
//const lm_instructions: &str = include_str!("lm_instruction.md");
//
//
//pub fn use_groq(msg: &str) -> anyhow::Result<String> {
//    let client = reqwest::Client::new();
//    let response = client.post("https://api.groq.com/openai/v1/chat/completions")
//        .json(&json!({
//            "model": "",
//            "messages": [
//                {
//                    "role": "system",
//                    "content": lm_instructions
//                },
//                {
//                    "role": "user",
//                    "content": msg
//                }
//            ],
//            "max_tokens": 8192,
//            "temperature": 0.0,
//            "top_p": 0.9,
//            "top_k": 1,
//            "stream": false,
//            "stop": null
//        }))
//        .send()?
//        .json()?;
//
//    Ok(response.choices[0].message.content.clone())
//}
//