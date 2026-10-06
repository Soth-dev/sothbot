use crate::{m, q, text};
use teloxide::{prelude::*, types::ParseMode};

pub async fn run(bot: Bot, msg: Message) -> anyhow::Result<()> {
    dbg!(&msg);
    text!(bot, msg, q!(m!("Done!")), ParseMode::Html).await?;
    Ok(())
}
