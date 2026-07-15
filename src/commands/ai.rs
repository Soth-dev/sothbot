use crate::{edit, m, q, text};
use dotenvy::dotenv;
use gemini_rust::{ClientError::BadResponse, Gemini, Model};
use std::env;
use teloxide::{prelude::*, types::ParseMode};

#[ctor::ctor(unsafe)]
static GEMINI_CLIENT: Gemini = {
    dotenv().unwrap();
    Gemini::with_model(env::var("GEMINI_API_KEY").unwrap(), Model::Gemini3Flash).unwrap()
};

pub async fn run(bot: Bot, msg: Message, text: String) -> anyhow::Result<()> {
    let reply_text = msg
        .quote()
        .map(|m| m.text.as_str())
        .or(msg.reply_to_message().and_then(|m| m.text()));
    if text.is_empty() && reply_text.is_none() {
        text!(bot, msg, "Use: /ai [your question]").await?;
        return Ok(());
    }
    let msg2 = text!(bot, msg, q!(m!("Generating...")), ParseMode::Html).await?;

    let mut content = GEMINI_CLIENT.create_interaction().with_text(text.trim());
    content = match reply_text {
        Some(t) => content.with_system_instruction(t),
        None => content,
    };

    let response = match content.execute().await {
        Ok(t) => t.output_text(),
        Err(e) => {
            dbg!(&e);
            if let BadResponse { description, .. } = e {
                println!(
                    "{}",
                    description
                        .as_deref()
                        .unwrap_or("Error: \x1b[91m(No details)\x1b[0m")
                )
            };
            ">`Failed to generate...`".to_string()
        }
    };

    let text_resp = sanitize_markdown(response);
    println!("\n{}", text_resp);
    if let Err(err) = edit!(bot, msg, msg2, text_resp, ParseMode::MarkdownV2).await {
        dbg!(err);
        edit!(
            bot,
            msg,
            msg2,
            q!(m!("Failed to generate...")),
            ParseMode::Html
        )
        .await?;
    }
    Ok(())
}

fn sanitize_markdown(text: String) -> String {
    text.replace(".", "\\.")
        .replace("!", "\\!")
        .replace("-", "\\-")
        .replace("+", "\\+")
        .replace("=", "\\=")
        .replace("#", "\\#")
        .replace("|", "\\|")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace(" >", " \\>")
        .replace("\\#\\#\\#", "●")
}
