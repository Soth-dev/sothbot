use std::{env, sync::LazyLock};

use crate::text;
use teloxide::{prelude::*, sugar::request::RequestLinkPreviewExt, types::ParseMode};

static SOURCE: LazyLock<String> =
    LazyLock::new(|| env::var("SOURCE_CODE").unwrap_or("None".to_string()));

pub async fn run(bot: Bot, msg: Message) -> anyhow::Result<()> {
    text!(bot, msg, &*SOURCE, ParseMode::MarkdownV2)
        .disable_link_preview(true)
        .await?;
    Ok(())
}
