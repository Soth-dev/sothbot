use crate::{delete, text, text_to};
use teloxide::prelude::*;

pub async fn run(bot: Bot, msg: Message, text: String) -> anyhow::Result<()> {
    if text.is_empty() {
        text!(
            bot,
            msg,
            "Please provide a message to echo!\nUsage: /echo [message]"
        )
        .await?;
        return Ok(());
    }

    if let Some(reply) = msg.reply_to_message() {
        text_to!(bot, msg, reply, text).await?;
    } else {
        let msg1 = text!(bot, msg, &text).await?;
        text!(bot, msg1, text).await?;
        delete!(bot, msg1).await?;
    }

    Ok(())
}
