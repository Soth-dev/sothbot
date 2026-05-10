use crate::text;
use teloxide::prelude::*;

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    text!(
        bot,
        msg,
        "Here are the things I can do:
    /joke - Get a random joke
    /ai [message] - Ask AI
    /echo [message] - Repeat what you say"
    )
    .await?;
    Ok(())
}
