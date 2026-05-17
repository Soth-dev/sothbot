use crate::{delete, text};
use serde::Deserialize;
use teloxide::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum JokeApiResponse {
    #[serde(rename = "single")]
    Single { joke: String },
    #[serde(rename = "twopart")]
    TwoPart { setup: String, delivery: String },
}

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    match joke().await {
        Ok(JokeApiResponse::Single { joke }) => {
            text!(bot, msg, joke).await?;
        }
        Ok(JokeApiResponse::TwoPart { setup, delivery }) => {
            text!(bot, msg, setup).await?;
            let msg1 = text!(bot, msg, &delivery).await?;
            text!(bot, msg1, delivery).await?;
            delete!(bot, msg1).await?;
        }
        Err(e) => {
            text!(bot, msg, "No joke found :(").await?;
            dbg!(e);
        }
    }

    Ok(())
}

async fn joke() -> Result<JokeApiResponse, Box<dyn std::error::Error + Sync + Send>> {
    let url = "https://v2.jokeapi.dev/joke/Any";
    let joke = reqwest::get(url).await?.json::<JokeApiResponse>().await?;
    Ok(joke)
}
