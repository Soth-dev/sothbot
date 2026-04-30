use gemini_rust::{ClientError::BadResponse, Gemini};
use std::{env, sync::OnceLock};
use teloxide::{prelude::*, types::ReplyParameters};

static GEMINI_CLIENT: OnceLock<Option<Gemini>> = OnceLock::new();

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = msg.text().unwrap_or("").replacen("/ai", "", 1);
    if text.trim().is_empty() {
        bot.send_message(msg.chat.id, "Use: /ai <your question>")
            .reply_parameters(ReplyParameters::new(msg.id))
            .await?;
        return Ok(());
    }

    let response = match GEMINI_CLIENT
        .get_or_init(|| Gemini::new(env::var("GEMINI_API_KEY").unwrap_or("null".to_string())).ok())
    {
        Some(client) => match client
            .generate_content()
            .with_user_message(text)
            .execute()
            .await
        {
            Ok(t) => t.text(),
            Err(e) => {
                if let BadResponse { description, .. } = e {
                    println!(
                        "{}",
                        description
                            .as_deref()
                            .unwrap_or("Error: \x1b[91m(No details)\x1b[0m")
                    )
                };
                "failed to request".to_string()
            }
        },
        None => "api key err".to_string(),
    };

    bot.send_message(msg.chat.id, response)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    Ok(())
}
