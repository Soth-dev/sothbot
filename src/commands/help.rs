use crate::{b, esp_html, i, text};
use teloxide::{prelude::*, types::ParseMode, utils::command::CommandDescriptions};

pub async fn run(bot: Bot, msg: Message, desc: CommandDescriptions<'static>) -> ResponseResult<()> {
    let help = desc
        .to_string()
        .split('\n')
        .map(|s| {
            let s = match esp_html!(s).split_once("—") {
                Some((c, t)) => format!("{} — {}", b!(c.trim()), i!(t.trim())),
                None => b!(s),
            };
            format!("  {}", s)
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
