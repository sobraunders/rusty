use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;

pub fn cmd_list(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        match db.list_user_data(username) {
            Ok(data) => {
                if data.is_empty() {
                    client.send_privmsg(&msg.channel, "No stored data")?;
                } else {
                    for (key, value) in data {
                        client.send_privmsg(&msg.channel, &format!("{}: {}", key, value))?;
                    }
                }
            }
            Err(e) => {
                client.send_privmsg(&msg.channel, &format!("Error listing data: {}", e))?;
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
    }
    Ok(())
}
