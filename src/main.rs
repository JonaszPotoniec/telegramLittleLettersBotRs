use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::sync::{atomic::Ordering, Arc};
use teloxide::prelude::*;

mod little_letters;
use little_letters::*;
mod logging;
use logging::*;

#[derive(Deserialize)]
struct Config {
    token: String,
    enable_logging: bool,
    logging_schedule: String,
    logging_filename: String,
}

#[tokio::main]
async fn main() {
    let config: Config = Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("CARGO_"))
        .merge(Env::raw().only(&["RUSTC", "RUSTDOC"]))
        .extract()
        .expect("Failed to load config");

    if config.token.len() == 0 {
        log::error!("Bot token has not been provided!");
        panic!("Bot token has not been provided!");
    }

    let counter: Option<LogHandle> = if config.enable_logging {
        create_and_start_counter(config.logging_filename, config.logging_schedule)
            .await
            .map(Some)
            .unwrap_or_else(|e| {
                log::error!("Failed to start logging: {}", e.message);
                None
            })
    } else {
        None
    };

    log::info!("Starting bot...");

    let bot = Bot::new(config.token);

    if let Some(c) = counter {
        teloxide::repl(bot, move |bot: Bot, message: Message| {
            let value = Arc::clone(&c.getData());

            async move {
                handle_message(bot, message)
                    .await
                    .unwrap_or_else(|e| log::error!("Error while handling message: {}", e));
                value.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        })
        .await;
    } else {
        teloxide::repl(bot, |bot: Bot, message: Message| async move {
            handle_message(bot, message)
                .await
                .unwrap_or_else(|e| log::error!("Error while handling message: {}", e));
            Ok(())
        })
        .await;
    }
}

async fn handle_message(
    bot: Bot,
    message: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(str) = message.text() {
        let little_letters = string_to_little_letters(str);
        bot.send_message(message.chat.id, little_letters).await?;
    }
    Ok(())
}
