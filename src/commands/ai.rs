use crate::func::{m, q};
use dotenvy::dotenv;
use gemini_rust::{ClientError::BadResponse, Gemini, Model};
use std::env;
use teloxide::{
    prelude::*,
    types::{ParseMode, ReplyParameters},
};

#[ctor::ctor]
static GEMINI_CLIENT: Gemini = {
    dotenv().unwrap();
    Gemini::with_model(env::var("GEMINI_API_KEY").unwrap(), Model::Gemini3Flash).unwrap()
};

pub async fn run(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    if text.trim().is_empty() {
        bot.send_message(msg.chat.id, "Use: /ai <your question>")
            .reply_parameters(ReplyParameters::new(msg.id))
            .await?;
        return Ok(());
    }
    let msg2 = bot
        .send_message(msg.chat.id, q(m("Generating...")))
        .parse_mode(ParseMode::Html)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    let reply_text = match msg.reply_to_message() {
        Some(m) => m.text(),
        None => None,
    };

    let mut content = GEMINI_CLIENT
        .generate_content()
        .with_user_message(text.trim());
    content = match reply_text {
        Some(t) => content.with_system_instruction(t),
        None => content,
    };

    let response = match content.execute().await {
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
    };

    bot.edit_message_text(msg.chat.id, msg2.id, sanitize_markdown(response))
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

fn sanitize_markdown(text: String) -> String {
    text.replace(".", "\\.")
        .replace("!", "\\!")
        .replace("-", "\\-")
        .replace("+", "\\+")
        .replace("=", "\\=")
        .replace(">", "\\>")
        .replace("#", "\\#")
        .replace("|", "\\|")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace("(", "\\(")
        .replace(")", "\\)")
}
