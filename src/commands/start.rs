use teloxide::prelude::*;
use teloxide::types::ReplyParameters;


pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    //println!("{:#?}", msg);
    bot.send_message(msg.chat.id, format!("Hello, {}!\n\nWelcome to the bot. Type /help to see what I can do!", msg.from.unwrap().first_name))
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    Ok(())
}
