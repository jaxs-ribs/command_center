
// fn _stt() -> anyhow::Result<()> {
//     let result = register_api_key("u no taek key");
//     println!("Result is going to be {:?}", result);

//     let audio = vec![0; 1024];
//     let result = openai_transcribe(&audio);
//     println!("Result is going to be {:?}", result);

//     Ok(())
// }

// // Update the llm() function to demonstrate the use of optional model parameters
// fn _llm() -> anyhow::Result<()> {
//     println!("Starting LLM operations");

//     // Register API keys
//     let api_keys = [
//         ("OpenAI", register_openai_api_key("sk-proj-J2y0MMBBYhLaw6iI680bT3BlbkFJPeH4fI3cumGNe6M6mbLX")),
//         ("Groq", register_groq_api_key("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")),
//         ("Claude", register_claude_api_key("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")),
//     ];

//     for (provider, result) in api_keys {
//         println!("{} API key registered: {:?}", provider, result);
//     }

//     // Get embedding
//     let embedding_result = embedding("This is an embedding test", None);
//     println!("Embedding result length: {:?}", embedding_result);

//     // Chat with different models
//     let chat_queries = [
//         ("OpenAI (default)", openai_chat("What is the capital of France?", None)),
//         ("OpenAI (GPT-4)", openai_chat("What is the capital of France?", Some("gpt-4"))),
//         ("Groq (default)", groq_chat("What is the capital of Germany?", None)),
//         ("Claude (default)", claude_chat("What is the capital of Japan?", None)),
//     ];

//     for (model, result) in chat_queries {
//         println!("{} chat result: {:?}", model, result);
//     }

//     Ok(())
// }

// fn _tg() -> anyhow::Result<()> {
//     let token_result = register_token("7327137177:AAHc5hXGmnUEI6CxrnlTYQTTGVG4Kphu288");
//     println!("CC: Token result: {:?}", token_result);

//     let sub_result = subscribe();
//     println!("CC: Sub result: {:?}", sub_result);

//     let mut counter = 0;
//     loop {
//         let message = await_message()?;
//         match message {
//             Message::Request { body, .. } => {
//                 let Ok(TgRequest::SendMessage(message)) = serde_json::from_slice(&body) else {
//                     return Err(anyhow::anyhow!("unexpected response: {:?}", body));
//                 };
//                 println!("CC: Message received: {:?}", message);
//                 let text = format!("The parrot said {}", message.text);
//                 let params = SendMessageParams {
//                     chat_id: message.chat_id,
//                     text,
//                 };
//                 let send_message_result = send_message(&params);
//                 println!("CC: Send message result: {:?}", send_message_result);
//                 counter += 1;
//                 if counter >= 10 {
//                     break;
//                 }
//             },
//             _ => {}
//         }
//     }

//     let unsub_result = unsubscribe();
//     println!("CC: Unsub result: {:?}", unsub_result);
//     Ok(())
// }

// fn test() -> anyhow::Result<()> {
//     // _stt()?;
//     // _llm()?;
//     // _tg()?;
//     Ok(())
// }