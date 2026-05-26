use crate::{m, q, text};
use teloxide::{prelude::*, types::ParseMode};

pub async fn md(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    if let Err(e) = text!(bot, msg, text, ParseMode::MarkdownV2).await {
        text!(bot, msg, q!(format!("Error: {}", m!(e))), ParseMode::Html).await?;
    };
    Ok(())
}

pub async fn html(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    if let Err(e) = text!(bot, msg, text, ParseMode::Html).await {
        text!(bot, msg, q!(format!("Error: {}", m!(e))), ParseMode::Html).await?;
    };
    Ok(())
}
