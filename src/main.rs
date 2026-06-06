mod chat_actions;
mod commands;
mod utils;

use commands::{Command, router};
use dotenvy::dotenv;
use teloxide::{filter_command, prelude::*, types::AllowedUpdate, update_listeners::Polling};

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    pretty_env_logger::init_timed();

    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .branch(filter_command::<Command, ResponseResult<()>>().endpoint(router))
                .branch(Message::filter_new_chat_members().endpoint(chat_actions::new_member))
                .branch(Message::filter_left_chat_member().endpoint(chat_actions::goodbye_member)),
        )
        .branch(
            Update::filter_chat_member()
                .branch(
                    dptree::filter(|m: ChatMemberUpdated| {
                        dbg!(&m);
                        m.old_chat_member.kind.is_left() && m.new_chat_member.kind.is_present()
                    })
                    .endpoint(chat_actions::new_member_50),
                )
                .branch(
                    dptree::filter(|m: ChatMemberUpdated| {
                        m.old_chat_member.kind.is_present() && m.new_chat_member.is_left()
                    })
                    .endpoint(chat_actions::goodbye_member_50),
                ),
        );
    let listener = Polling::builder(bot.clone())
        .allowed_updates(vec![AllowedUpdate::ChatMember, AllowedUpdate::Message])
        .build();

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, LoggingErrorHandler::new())
        .await;
}
