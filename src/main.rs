mod chat_actions;
mod commands;
mod utils;

use commands::{Command, router};
use dotenvy::dotenv;
use teloxide::{dispatching::filter_command, prelude::*};

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    pretty_env_logger::init_timed();

    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry().branch(
        Update::filter_message()
            .branch(filter_command::<Command, ResponseResult<()>>().endpoint(router))
            .branch(Message::filter_new_chat_members().endpoint(chat_actions::new_member))
            .branch(Message::filter_left_chat_member().endpoint(chat_actions::goodbye_member)),
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
