automod::dir!("src/commands");

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Welcome message.")]
    Start,
    #[command(description = "Show this help message.")]
    Help,
    #[command(description = "Repeat your message.")]
    Echo { text: String },
    #[command(description = "Send a random joke.")]
    Joke,
    #[command(description = "Ask AI.")]
    Ai { text: String },
    #[command(description = "Create a maze.")]
    Maze { text: String },
    #[command(description = "Write inside a book.")]
    Write { text: String },
    #[command(description = "Flip text.")]
    Flip { text: String },
    #[command(description = "HTML format.")]
    Html { text: String },
    #[command(description = "Markdown V2 format.")]
    Md { text: String },
    #[command(description = "Meowing for you.")]
    Meow,
}

pub async fn router(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    tokio::spawn(async move {
        if let Err(e) = command_router(bot, msg, cmd).await {
            log::error!(target: "RequestError", "{:#?}", e);
        }
    });
    Ok(())
}

async fn command_router(bot: Bot, msg: Message, cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Start => start::run(bot, msg).await,
        Command::Help => help::run(bot, msg, Command::descriptions()).await,
        Command::Echo { text } => echo::run(bot, msg, text).await,
        Command::Joke => joke::run(bot, msg).await,
        Command::Ai { text } => ai::run(bot, msg, text).await,
        Command::Maze { text } => maze::maze(bot, msg, text).await,
        Command::Write { text } => write_note::run(bot, msg, text).await,
        Command::Flip { text } => texting::flip(bot, msg, text).await,
        Command::Html { text } => formats::html(bot, msg, text).await,
        Command::Md { text } => formats::md(bot, msg, text).await,
        Command::Meow => meow::run(bot, msg).await,
    }
}
