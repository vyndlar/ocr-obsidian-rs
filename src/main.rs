use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

// The 'comments' below are actually displayed to the user
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    /// This message, add a command name to get help with a specific command
    /// Example ````/help ls``` would give help for the 'List' command
    #[command(aliases = ["h", "?"])]
    Help { help_command: String },

    /// List all vaults
    #[command(aliases = ["ls"])]
    List,

    /// Settings Panel
    #[command()]
    Settings,

    /// Add a new vault
    #[command()]
    New { data: String },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help { help_command } => {
            log::info!("Help command");
            if help_command.is_empty() {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?
            } else {
                bot.send_message(
                    msg.chat.id,
                    get_help_with_command(help_command.to_lowercase().as_str()),
                )
                .await?
            }
        }
        Command::List => {
            log::info!("List Command");
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Settings => {
            log::info!("Settings");
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::New { data } => {
            bot.send_message(
                msg.chat.id,
                format!("{}, {}", data, Command::descriptions()),
            )
            .await?
        }
    };

    Ok(())
}

fn get_help_with_command(command: &str) -> String {
    match command {
        "list" | "ls" => {
            "The ```list``` function listst all vaults that ocr-obsidian can see. If one you want is not here, try using the ```/add``` command.".to_string()
        },

        _ => "Error".to_string(),
    }
}
