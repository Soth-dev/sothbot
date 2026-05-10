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
            text!(bot, msg, joke).await?;
        }
        Some(JokeApiResponse::TwoPart { setup, delivery }) => {
            text!(bot, msg, setup).await?;
            let msg1 = text!(bot, msg, &delivery).await?;
            text!(bot, msg1, delivery).await?;
            delete!(bot, msg1).await?;
        }
        None => {
            text!(bot, msg, "No joke found :(").await?;
        }
    }

    Ok(())
}
