use teloxide::{prelude::*, types::ReplyParameters};

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        "Here are the things I can do:
    /joke - Get a random joke
    /ai [message] - Ask AI
    /echo [message] - Repeat what you say")
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    Ok(())
}
