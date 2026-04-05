use gemini_rust::Gemini;
use std::env;
use teloxide::{prelude::*, types::ReplyParameters};

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    let api_key = env::var("GEMINI_API_KEY").expect("Gemini API Key not set");
    let client = Gemini::new(&api_key).expect("Key is invalid");

    let text = msg.text().unwrap_or("").trim();
    let mut parts = text.split_whitespace();
    let command = parts.next().unwrap_or("");
    let user_message = parts.collect::<Vec<_>>().join(" ");

    if !command.starts_with("/ai") || user_message.trim().is_empty() {
        bot.send_message(msg.chat.id, "Use: /ai <your question>")
            .reply_parameters(ReplyParameters::new(msg.id))
            .await?;
        return Ok(());
    }

    let response = client
        .generate_content()
        .with_user_message(&user_message)
        .execute()
        .await
        .expect("No response available");

    bot.send_message(msg.chat.id, response.text())
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    Ok(())
}
