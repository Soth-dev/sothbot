use crate::text;
use teloxide::prelude::*;

pub async fn run(bot: Bot, msg: Message) -> anyhow::Result<()> {
    let name = match msg.from {
        Some(u) => u.first_name,
        None => "there".to_string(),
    };
    text!(
        bot,
        msg,
        format!(
            "Hello, {}!\n\nWelcome to the bot. Type /help to see what I can do! 🚀",
            name
        )
    )
    .await?;
    Ok(())
}
