use teloxide::prelude::*;
use teloxide::types::ReplyParameters;

pub async fn run(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    if text.is_empty() {
        bot.send_message(
            msg.chat.id,
            "Please provide a message to echo!\nUsage: /echo [message]",
        )
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
        return Ok(());
    }
    let replied = msg.reply_to_message();

    match replied {
        Some(reply) => {
            bot.send_message(msg.chat.id, text)
                .reply_parameters(ReplyParameters::new(reply.id))
                .await?;
        }
        None => {
            let msg1 = bot
                .send_message(msg.chat.id, &text)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?
                .id;
            bot.send_message(msg.chat.id, text)
                .reply_parameters(ReplyParameters::new(msg1))
                .await?;
            bot.delete_message(msg.chat.id, msg1).await?;
        }
    };
    Ok(())
}
