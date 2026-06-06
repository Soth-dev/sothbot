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

pub async fn new_member_50(bot: Bot, cm: ChatMemberUpdated) -> ResponseResult<()> {
    let user = cm.new_chat_member.user.first_name;
    bot.send_message(cm.chat.id, format!("Welcome, {}!", b!(esp_html!(user))))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn goodbye_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(user) = msg.left_chat_member() {
        text!(
            bot,
            msg,
            format!("Goodbye, {}! 😢", b!(esp_html!(user.first_name))),
            ParseMode::Html
        )
        .await?;
    }
    Ok(())
}

pub async fn goodbye_member_50(bot: Bot, cm: ChatMemberUpdated) -> ResponseResult<()> {
    let user = cm.new_chat_member.user.first_name;
    bot.send_message(cm.chat.id, format!("Goodbye, {}! 😞", b!(esp_html!(user))))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}
