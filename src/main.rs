mod message;
mod database;
mod commands;
mod games;

use irc::client::prelude::*;
use std::default::Default;
use futures::TryStreamExt;
use message::ParsedMessage;
use database::Database;
use commands::handle_command;
use games::GameManager;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let db = Database::new("bot_data.db")
        .expect("Failed to initialize database");

    let games = GameManager::new();

    let config = Config {
        nickname: Some("rusty".to_string()),
        server: Some("irc.moparisthebest.com".to_string()),
        port: Some(6697),
        use_tls: Some(true),
        channels: vec!["#testes".to_string()],
        ..Default::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.try_next().await? {
        println!("{}", message);

        match message.command {
            Command::PRIVMSG(ref channel, ref text) => {
                let author = message.source_nickname().map(|s| s.to_string());
                let parsed = ParsedMessage::parse(&text, author.clone(), channel.clone());

                if parsed.is_command() {
                    handle_command(&client, &parsed, &db, &games).await?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
