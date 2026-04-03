use serde::Deserialize;
use teloxide::prelude::*;
use teloxide::types::ReplyParameters;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum JokeApiResponse {
    #[serde(rename = "single")]
    Single { joke: String },
    #[serde(rename = "twopart")]
    TwoPart { setup: String, delivery: String },
}

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    let url = "https://v2.jokeapi.dev/joke/Any";
    let joke = match reqwest::get(url).await {
        Ok(v) => match v.json::<JokeApiResponse>().await {
            Ok(t) => Some(t),
            Err(e) => {
                println!("{e:#?}");
                None
            }
        },
        Err(e) => {
            println!("{e:#?}");
            None
        }
    };

    match joke {
        Some(JokeApiResponse::Single { joke }) => {
            bot.send_message(msg.chat.id, joke)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
        }
        Some(JokeApiResponse::TwoPart { setup, delivery }) => {
            bot.send_message(msg.chat.id, setup)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
            bot.send_message(msg.chat.id, delivery)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "No joke found :(")
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
        }
    }

    Ok(())
}
