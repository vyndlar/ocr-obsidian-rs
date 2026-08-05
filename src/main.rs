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

    // command handler to handle different command types
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help { help_command }].endpoint(help_command_handler))
        .branch(case![Command::List].endpoint(list_command_handler))
        .branch(case![Command::Settings].endpoint(settings_command_handler))
        .branch(case![Command::New { data }].endpoint(new_command_handler));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        // text-only messages
        .branch(Message::filter_text().endpoint(text_message_handler))
        .branch(Message::filter_photo().endpoint(photo_message_handler));

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

async fn list_command_handler(bot: Bot, msg: Message) -> HandlerResult {
    log::info!("List Command");
    bot.send_message(msg.chat.id, "The list command").await?;

    Ok(())
}

async fn settings_command_handler(bot: Bot, msg: Message) -> HandlerResult {
    log::info!("Settings");
    bot.send_message(msg.chat.id, "The settings command")
        .await?;

    Ok(())
}

async fn new_command_handler(bot: Bot, msg: Message, data: String) -> HandlerResult {
    if data.is_empty() {
        bot.send_message(msg.chat.id, "The new command, with no data!")
            .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            format!("The new command, here is your data: , {}", data),
        )
        .await?;
    }
    Ok(())
}

async fn text_message_handler(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "You sent a text message!")
        .await?;
    Ok(())
}

async fn photo_message_handler(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "You sent a photo!").await?;
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
