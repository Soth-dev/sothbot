use crate::text;
use teloxide::prelude::*;

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(u) = msg.from {
        text!(
            bot,
            msg,
            format!(
                "Hello, {}!\n\nWelcome to the bot. Type /help to see what I can do!",
                u.first_name
            )
        )
        .await?;
    } else {
        text!(
            bot,
            msg,
            "Hello, there!\n\nWelcome to the bot. Type /help to see what I can do!"
        )
        .await?;
    }
    Ok(())
}
