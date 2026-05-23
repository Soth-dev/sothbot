pub mod ai;
pub mod echo;
pub mod help;
pub mod joke;
pub mod maze;
pub mod start;
mod texting;
mod write_note;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    Help,
    Echo { text: String },
    Joke,
    Ai { text: String },
    Maze { text: String },
    Write { text: String },
    Flip { text: String },
}

pub async fn router(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    tokio::spawn(async move {
        if let Err(e) = command_router(bot, msg, cmd).await {
            print!("\x1b[101m\x1b[93m");
            dbg!(e);
            println!("\x1b[0m");
        }
    });
    Ok(())
}

pub async fn command_router(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => start::run(bot, msg).await,
        Command::Help => help::run(bot, msg).await,
        Command::Echo { text } => echo::run(bot, msg, text).await,
        Command::Joke => joke::run(bot, msg).await,
        Command::Ai { text } => ai::run(bot, msg, text).await,
        Command::Maze { text } => maze::maze(bot, msg, text).await,
        Command::Write { text } => write_note::run(bot, msg, text).await,
        Command::Flip { text } => texting::flip(bot, msg, text).await,
    }
}
