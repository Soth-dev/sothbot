use crate::{delete, text};
use serde::Deserialize;
use teloxide::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum JokeResponse {
    #[serde(rename = "single")]
    Single { joke: String },
    #[serde(rename = "twopart")]
    TwoPart { setup: String, delivery: String },
}

pub async fn run(bot: Bot, msg: Message) -> ResponseResult<()> {
    match joke().await {
        Ok(JokeResponse::Single { joke }) => {
            text!(bot, msg, joke).await?;
        }
        Ok(JokeResponse::TwoPart { setup, delivery }) => {
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

async fn joke() -> Result<JokeResponse, reqwest::Error> {
    reqwest::get("https://v2.jokeapi.dev/joke/Any")
        .await?
        .json::<JokeResponse>()
        .await
}
