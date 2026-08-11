use crate::{edit, m, q, text};
use dotenvy::dotenv;
use gemini_rust::{Gemini, Model};
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

    match ai(text, reply_text).await {
        Err(e) => {
            edit!(
                bot,
                msg,
                msg2,
                ">`Failed to generate...`".to_string(),
                ParseMode::MarkdownV2
            )
            .await
            .unwrap(); // 100% no err... i swear...
            Err(e)
        }
        Ok(resp) => {
            let text_resp = sanitize_markdown(resp);
            println!("\n{}", text_resp);
            if let Err(err) = edit!(bot, msg, msg2, text_resp, ParseMode::MarkdownV2).await {
                edit!(
                    bot,
                    msg,
                    msg2,
                    q!(m!("Failed to generate...")),
                    ParseMode::Html
                )
                .await?;
                return Err(anyhow::Error::new(err));
            }

            Ok(())
        }
    }
}

async fn ai(text: String, reply_text: Option<&str>) -> anyhow::Result<String> {
    let mut content = GEMINI_CLIENT.create_interaction().with_text(text.trim());
    content = match reply_text {
        Some(t) => content.with_system_instruction(t),
        None => content,
    };

    let response = content.execute().await?.output_text();
    Ok(response)
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
