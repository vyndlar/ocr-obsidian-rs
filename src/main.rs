// TODO: Add user state enum
use dptree::case;
use teloxide::{
    prelude::*,
    types::{Message, Update},
    utils::command::BotCommands,
};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type HandlerResult = Result<(), Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    log::info!("Starting bot");

    let bot = Bot::from_env();

    //Command::repl(bot, answer).await;

    // command handler to handle different command types
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help { help_command }].endpoint(help_command_handler));

    let message_handler = Update::filter_message().branch(command_handler);

    let schema = message_handler;

    Dispatcher::builder(bot, schema)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

// The 'comments' below are actually display to the user
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    /// This message, add a command name to get help with a specific command
    /// Example ```/help ls``` would give help for the 'List' command
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

async fn help_command_handler(bot: Bot, msg: Message, help_command: String) -> HandlerResult {
    log::info!("Help command");
    if help_command.is_empty() {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            get_help_with_command(&help_command.to_lowercase()),
        )
        .await?;
    }
    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    log::info!("Msg recieved!");

    if msg.text().is_some() {
        text_message(bot, msg, cmd).await?;
    } else if msg.photo().is_some() {
        photo_msg(bot, msg).await?;
    } else {
        log::error!("Message type not captured in answer()");
    }

    Ok(())
}

async fn text_message(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
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

async fn photo_msg(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Photo".to_string()).await?;
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
