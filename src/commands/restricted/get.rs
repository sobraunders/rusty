use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;

pub fn cmd_get(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if msg.args.is_empty() {
            client.send_privmsg(&msg.channel, "Usage: !get <key>")?;
        } else {
            let key = &msg.args[0];
            
            match db.get_user_data(username, key) {
                Ok(Some(value)) => {
                    client.send_privmsg(&msg.channel, &format!("{} = {}", key, value))?;
                }
                Ok(None) => {
                    client.send_privmsg(&msg.channel, &format!("Key not found: {}", key))?;
                }
                Err(e) => {
                    client.send_privmsg(&msg.channel, &format!("Error retrieving data: {}", e))?;
                }
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
    }
    Ok(())
}
