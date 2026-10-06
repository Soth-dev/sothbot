use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton as Key, InlineKeyboardMarkup},
};

pub async fn run(bot: Bot, msg: Message) -> anyhow::Result<()> {
    bot.send_message(msg.chat.id, "Choose a game:")
        .reply_markup(
            InlineKeyboardMarkup::default()
                .append_row(vec![Key::callback("Last Word", "game_lastword")]),
        )
        .await?;
    Ok(())
}
