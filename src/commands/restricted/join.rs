use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;
use crate::commands::utils::has_permission;

pub fn cmd_join(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if !has_permission(db, username) {
            client.send_privmsg(&msg.channel, "Permission denied. You don't have permission to use this command")?;
            return Ok(());
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
        return Ok(());
    }

    if msg.args.is_empty() {
        client.send_privmsg(&msg.channel, "Usage: !join <#channel>")?;
    } else {
        let channel = &msg.args[0];
        if !channel.starts_with('#') {
            client.send_privmsg(&msg.channel, "Channel name must start with #")?;
        } else {
            client.send_join(channel)?;
            client.send_privmsg(&msg.channel, &format!("Joining {}", channel))?;
        }
    }
    Ok(())
}
