use crate::{
    edit,
    func::{m, q},
    text,
};
use dotenvy::dotenv;
use gemini_rust::{ClientError::BadResponse, Gemini, Model};
use std::env;
use teloxide::{prelude::*, types::ParseMode};

#[ctor::ctor]
static GEMINI_CLIENT: Gemini = {
    dotenv().unwrap();
    Gemini::with_model(env::var("GEMINI_API_KEY").unwrap(), Model::Gemini3Flash).unwrap()
};

pub async fn run(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    if text.trim().is_empty() {
        text!(bot, msg, "Use: /ai <your question>")?;
        return Ok(());
    }
    let msg2 = text!(bot, msg, q(m("Generating...")), ParseMode::Html)?;
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

    edit!(
        bot,
        msg,
        msg2,
        sanitize_markdown(response),
        ParseMode::MarkdownV2
    )?;
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
