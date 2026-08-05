use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

// The 'comment' is actually what displays to the user
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    /// This message
    #[command(aliases = ["h", "?"])]
    Help,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            log::info!("Help command");
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
    };

    Ok(())
}
