use crate::{b, esp_html, text};
use teloxide::{prelude::*, types::ParseMode};
pub async fn new_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(users) = msg.new_chat_members().map(|l| {
        l.iter()
            .map(|u| b!(esp_html!(u.first_name.clone())))
            .collect::<Vec<String>>()
    }) {
        text!(
            bot,
            msg,
            format!("Welcome, {}!", users.join(", ")),
            ParseMode::Html
        )
        .await?;
    }
    Ok(())
}
pub async fn goodbye_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(users) = msg.new_chat_members().map(|l| {
        l.iter()
            .map(|u| b!(esp_html!(u.first_name.clone())))
            .collect::<Vec<String>>()
    }) {
        text!(
            bot,
            msg,
            format!("Goodbye, {}! 😢", users.join(", ")),
            ParseMode::Html
        )
        .await?;
    }
    Ok(())
}
