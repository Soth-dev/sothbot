use crate::{m, q, text};
use teloxide::prelude::*;

pub async fn run(bot: Bot, msg: Message) -> anyhow::Result<()> {
    dbg!(&msg);
    text!(bot, msg, q!(m!("Done!"))).await?;
    Ok(())
}
