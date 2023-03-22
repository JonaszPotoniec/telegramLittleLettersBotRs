use cron::Schedule;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::str::FromStr;
use teloxide::prelude::*;
use teloxide::*;

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
    // logging::enable_logging!();

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
    println!("1");

    if config.enable_logging {
        let log_handle = LogHandle::new(
            config.logging_filename,
            Schedule::from_str(&config.logging_schedule.to_owned()).unwrap(),
        )
        .await;
        match log_handle {
            Ok(handle) => {
                let result = handle.start().await;
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Failed to start logging: {}", e.message);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to start logging: {}", e.message);
            }
        }
    }
    println!("2");

    log::info!("Starting bot...");

    let bot = Bot::new(config.token);

    teloxide::repl(bot, |bot: Bot, message: Message| async move {
        match message.text() {
            Some(str) => {
                let little_letters = string_to_little_letters(str);
                bot.send_message(message.chat.id, little_letters).await?;
            }
            _ => {}
        }
        respond(())
    })
    .await;
}
