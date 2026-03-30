use teloxide::prelude::*;
use teloxide::types::ReplyParameters;


pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    let url = "https://v2.jokeapi.dev/joke/Any";
    let res = match reqwest::get(url).await {
        Ok(v) => match v.text().await {
            Ok(t) => t,
            e =>{
                println!("{e:#?}");
                "err".to_string()
            }
        },
        e => {
            println!("{e:#?}");
            "err".to_string()
        }
    };
    println!("{:#?}", res);
    bot.send_message(msg.chat.id, res)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    Ok(())
}
