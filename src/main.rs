mod commands;

use teloxide::prelude::*;
use dotenvy::dotenv;
use commands::{Command, command_router};

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

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

