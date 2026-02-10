use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;

pub fn cmd_set(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if msg.args.len() < 2 {
            client.send_privmsg(&msg.channel, "Usage: !set <key> <value>")?;
        } else {
            let key = &msg.args[0];
            let value = msg.args[1..].join(" ");
            
            match db.set_user_data(username, key, &value) {
                Ok(_) => {
                    client.send_privmsg(&msg.channel, &format!("Saved: {} = {}", key, value))?;
                }
                Err(e) => {
                    client.send_privmsg(&msg.channel, &format!("Error saving data: {}", e))?;
                }
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
    }
    Ok(())
}
