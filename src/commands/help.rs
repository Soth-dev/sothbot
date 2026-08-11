use crate::{b, esp_html, i, text};
use teloxide::{prelude::*, types::ParseMode, utils::command::CommandDescriptions};

pub async fn run(bot: Bot, msg: Message, desc: CommandDescriptions<'static>) -> anyhow::Result<()> {
    let help = desc
        .to_string()
        .split('\n')
        .filter_map(|s| match esp_html!(s).split_once("—") {
            Some((_, t)) if t.starts_with('!') => None,
            Some((c, t)) => Some(format!("  {} — {}", b!(c.trim()), i!(t.trim()))),
            None => Some(format!("  {}", b!(s))),
        })
        .collect::<Vec<String>>()
        .join("\n");
    text!(
        bot,
        msg,
        format!("Here are the things I can do:\n{}", help),
        ParseMode::Html
    )
    .await?;
    Ok(())
}
