use crate::{b, esp_html, text};
use dotenvy::dotenv;
use std::env;
use teloxide::{prelude::*, types::ParseMode};

#[ctor::ctor(unsafe)]
static WELCOME_MEMBER: bool = {
    dotenv().unwrap();
    env::var("WELCOME_MESSAGE").unwrap() == "true"
};

#[ctor::ctor(unsafe)]
static GOODBYE_MESSAGE: bool = {
    dotenv().unwrap();
    env::var("GOODBYE_MESSAGE").unwrap() == "true"
};

pub async fn new_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    if !*WELCOME_MEMBER {
        return Ok(());
    }
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
    if !*WELCOME_MEMBER {
        return Ok(());
    }
    let user = cm.new_chat_member.user.first_name;
    bot.send_message(cm.chat.id, format!("Welcome, {}!", b!(esp_html!(user))))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn goodbye_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    if !*GOODBYE_MESSAGE {
        return Ok(());
    }
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
    if !*GOODBYE_MESSAGE {
        return Ok(());
    }
    let user = cm.new_chat_member.user.first_name;
    bot.send_message(cm.chat.id, format!("Goodbye, {}! 😞", b!(esp_html!(user))))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}
