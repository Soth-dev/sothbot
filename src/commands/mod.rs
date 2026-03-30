pub mod start;
pub mod help;
pub mod echo;
pub mod joke;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    Help,
    Echo,
    Joke
}

pub async fn command_router(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => start::run(bot, msg).await,
        Command::Help => help::run(bot, msg).await,
        Command::Echo => echo::run(bot, msg).await,
        Command::Joke => joke::run(bot, msg).await,
    }
}
