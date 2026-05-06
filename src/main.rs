mod commands;
pub mod func;
mod utils;

use commands::{Command, command_router};
use dotenvy::dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot: Bot = Bot::from_env();

    let handler: Handler<
        '_,
        Result<(), teloxide::RequestError>,
        teloxide::dispatching::DpHandlerDescription,
    > = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(command_router);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
