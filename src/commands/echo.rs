use teloxide::prelude::*;
use teloxide::types::ReplyParameters;


pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    let (text, suss) = msg.text()
        .unwrap()
        .split_once(" ")
        .map(|(_cmd, arg)| (arg, true))
        .unwrap_or(("Please provide a message to echo!\nUsage: /echo [message]", false));
    //println!("{:#?}", msg);
    //println!("{:#?}", msg.reply_to_message().is_some());
    //println!("{:#?}", msg.reply_to_message().unwrap_or("error!").id);
    let msg1 = bot.send_message(msg.chat.id, text)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    let reply_id = match msg.reply_to_message() {
        Some(reply) => reply.id,
        None => msg1.id
    };
    //println!("{:#?}",reply_text);
    if suss {
        bot.send_message(msg.chat.id, text)
            .reply_parameters(ReplyParameters::new(reply_id))
            .await?;
        bot.delete_message(msg1.chat.id, msg1.id)
            .await?;
    };
    Ok(())
}
