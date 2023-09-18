use chatgpt::prelude::*;

use super::utils::get_file_path;

fn chatgpt_client() -> ChatGPT {
    let client = ChatGPT::new("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");

    client.unwrap()
}
pub fn send_message(message: String, unique_id: String) -> String {
    let client = chatgpt_client();

    let _message = limit_message_to_tokens(&message, 1000);

    let mut conversation;

    // if !unique_id.is_empty() {
    //     conversation = match client
    //         .restore_conversation_json(get_file_path(&unique_id, "json"))
    //         .await
    //     {
    //         Ok(conversation) => conversation,
    //         Err(e) => {
    //             println!("erro ao restaurar conversa, err: {}", e);
    //             client.new_conversation()
    //         }
    //     }
    // } else {
    conversation = client.new_conversation();
    // }

    // println!("message 222222: {}", message);

    // let resp = conversation.send_message(message).await;
    println!("555555555555555555555555");
    let resp = futureconversation
        .send_message("qual o maior panela do mundo?")
        .await;

    println!("777777777777777777777777777777");
    println!("resp: {:?}", resp);
    println!("!unique_id.is_empty() {}", !unique_id.is_empty());

    if !unique_id.is_empty() {
        println!("unique_id: {} 33333333", unique_id);

        conversation
            .save_history_json(get_file_path(&unique_id, "json"))
            .await
            .unwrap();
    }

    resp.unwrap().message().content.clone()
}

fn limit_message_to_tokens(message: &str, max_tokens: usize) -> String {
    let tokens: Vec<&str> = message.split_whitespace().collect();

    if tokens.len() <= max_tokens {
        // Se a mensagem já tem 4000 tokens ou menos, não é necessário reduzir.
        return message.to_string();
    }

    // Se a mensagem tem mais de 4000 tokens, vamos selecionar os primeiros 4000 tokens.
    let limited_tokens: Vec<&str> = tokens.iter().take(max_tokens).cloned().collect();

    limited_tokens.join(" ")
}
