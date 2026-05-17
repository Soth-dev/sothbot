mod commands;
mod utils;

use commands::{Command, command_router};
use dotenvy::dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(command_router);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
