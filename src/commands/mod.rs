pub mod ai;
pub mod echo;
pub mod help;
pub mod joke;
pub mod maze;
pub mod start;

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
}

pub async fn command_router(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => start::run(bot, msg).await,
        Command::Help => help::run(bot, msg).await,
        Command::Echo { text } => echo::run(bot, msg, text).await,
        Command::Joke => joke::run(bot, msg).await,
        Command::Ai { text } => ai::run(bot, msg, text).await,
        Command::Maze { text } => maze::maze(bot, msg, text).await,
    }
}
