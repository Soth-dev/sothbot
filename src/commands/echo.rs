use crate::{delete, text, text_to};
use teloxide::prelude::*;

pub async fn run(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    if text.is_empty() {
        text!(
            bot,
            msg,
            "Please provide a message to echo!\nUsage: /echo [message]"
        )?;
        return Ok(());
    }

    if let Some(reply) = msg.reply_to_message() {
        text_to!(bot, msg, reply, text)?;
    } else {
        let msg1 = text!(bot, msg, &text)?;
        text!(bot, msg1, text)?;
        delete!(bot, msg1)?;
    }

    Ok(())
}
